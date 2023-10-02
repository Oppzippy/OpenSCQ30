use std::{
    env,
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
    sync::Once,
};

use actions::{State, StateUpdate};
use adw::Toast;
use anyhow::{anyhow, Context};
use gtk::{
    gdk::Display,
    gio::{self, SimpleAction},
    glib::{self, clone, MainContext, OptionFlags, Priority},
    prelude::*,
    traits::GtkWindowExt,
    CssProvider,
};
use logging_level::LoggingLevel;
use openscq30_lib::api::{device::DeviceRegistry, new_soundcore_device_registry};
use settings::Settings;
use tracing::Level;
use ui::widgets::MainWindow;
#[cfg(target_os = "windows")]
use windows::{
    core::IInspectable,
    Foundation::TypedEventHandler,
    UI::ViewManagement::{UIColorType, UISettings},
};

use crate::{actions::Action, gtk_futures::GtkFutures, objects::CustomEqualizerProfileObject};

mod actions;
mod gettext;
mod gettext_sys;
mod gtk_futures;
mod logging_level;
#[cfg(test)]
mod mock;
mod objects;
mod settings;
mod swappable_broadcast;
#[allow(clippy::new_without_default)]
mod ui;

pub struct ApplicationId<'a> {
    pub qualifier: &'a str,
    pub organization: &'a str,
    pub application: &'a str,
}
pub static APPLICATION_ID: ApplicationId = ApplicationId {
    qualifier: "com",
    organization: "oppzippy",
    application: "OpenSCQ30",
};
pub static APPLICATION_ID_STR: &str = "com.oppzippy.OpenSCQ30";

fn main() {
    // tracing is not set up yet, so we have to use println
    if let Err(err) = set_up_gettext() {
        eprintln!("failed to set up gettext, using default locale: {err}");
    }
    load_resources();
    run_application();
}

fn set_up_gettext() -> anyhow::Result<()> {
    match gettext::setlocale(0 /* LC_ALL */, "") {
        Some(selected_locale) => tracing::info!("selected locale: {selected_locale}"),
        None => eprintln!("failed to set locale"),
    }
    let locale_dir = get_locale_dir()?;
    #[cfg(debug_assertions)]
    eprintln!("found locale dir: {locale_dir:?}");
    gettext::bindtextdomain(APPLICATION_ID_STR, locale_dir.to_str().unwrap())?;
    gettext::bind_textdomain_codeset(APPLICATION_ID_STR, "UTF-8")?;
    gettext::textdomain(APPLICATION_ID_STR)?;
    Ok(())
}

fn get_locale_dir() -> anyhow::Result<PathBuf> {
    let current_exe = env::current_exe()?;
    let executable_dir = current_exe
        .parent()
        .ok_or_else(|| anyhow!("current_exe has no parent directory"))?;

    check_locale_dir(executable_dir)
        .or_else(|| executable_dir.parent().and_then(check_locale_dir))
        .ok_or_else(|| anyhow!("could not find locale dir"))
}

fn check_locale_dir(path: &Path) -> Option<PathBuf> {
    let locale_path = path.join("share").join("locale");
    if locale_path.is_dir() {
        Some(locale_path)
    } else {
        None
    }
}

static LOAD_RESOURCES: Once = Once::new();

pub fn load_resources() {
    LOAD_RESOURCES.call_once(|| {
        gio::resources_register_include!("widgets.gresource").expect("failed to load widgets");
    });
}

fn run_application() {
    let app = adw::Application::builder()
        .application_id(APPLICATION_ID_STR)
        .build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    handle_command_line_args(&app);

    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/com/oppzippy/OpenSCQ30/ui/style.css");
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn handle_command_line_args<T>(application: &T)
where
    T: IsA<gtk::Application> + IsA<gtk::gio::Application>,
{
    application.add_main_option(
        "logging-level",
        // Do not instantiate glib::Char(i8) directly, since on arm64 it's glib::Char(u8) instead
        b'l'.into(),
        OptionFlags::NONE,
        glib::OptionArg::String,
        &format!("Logging Level {}", LoggingLevel::allowed_values_string()),
        Some("LEVEL"),
    );

    application.connect_handle_local_options(|_application, options| {
        let maybe_logging_level = options
            .lookup::<String>("logging-level")
            .expect("logging-level must be a string")
            .map(|logging_level| {
                LoggingLevel::from_str(&heck::AsUpperCamelCase(logging_level).to_string())
            });

        let logging_level = match maybe_logging_level {
            Some(Ok(logging_level)) => logging_level,
            Some(Err(err)) => {
                println!(
                    "Invalid logging level: {err}. Allowed values: {}",
                    LoggingLevel::allowed_values_string(),
                );
                return 1; // Non-negative number means exit application
            }
            None => {
                // In debug builds, the default logging level is lower for convenience
                #[cfg(debug_assertions)]
                {
                    LoggingLevel::Trace
                }
                #[cfg(not(debug_assertions))]
                {
                    LoggingLevel::Info
                }
            }
        };
        tracing_subscriber::fmt()
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_max_level(Level::from(logging_level))
            .pretty()
            .init();
        -1 // Ok, proceed with starting application
    });
}

fn build_ui(application: &adw::Application) {
    // Display ui while initializing DeviceRegistry asynchronously
    #[cfg(target_os = "windows")]
    if let Err(err) = set_ui_theme(application) {
        tracing::warn!("failed to set ui theme: {err:?}");
    }
    let settings: Rc<Settings> = Default::default();
    if let Err(err) = settings.load() {
        tracing::warn!("initial load of settings file failed: {:?}", err)
    }
    let main_window = MainWindow::new(application, settings.to_owned());

    settings
        .config
        .get(|config| {
            main_window.set_custom_profiles(
                config
                    .custom_profiles()
                    .iter()
                    .map(|(name, profile)| {
                        CustomEqualizerProfileObject::new(name, profile.volume_adjustments())
                    })
                    .collect(),
            );
        })
        .unwrap();

    {
        let application = application.to_owned();
        let main_window = main_window.to_owned();
        MainContext::default().spawn_local(async move {
            let registry = new_soundcore_device_registry::<GtkFutures>()
                .await
                .expect("failed to initialize device registry");
            // Async initialization done, now set up event handlers and such
            delayed_initialize_application(&application, &main_window, registry, settings);
        });
    }

    main_window.present();
}

fn delayed_initialize_application(
    application: &adw::Application,
    main_window: &MainWindow,
    registry: impl DeviceRegistry + 'static,
    settings: Rc<Settings>,
) {
    let (state, mut ui_state_receiver) = State::new(registry);
    let state = Rc::new(state);
    let settings = Rc::new(settings);

    let main_context = MainContext::default();
    main_context.spawn_local(clone!(@weak main_window => async move {
        loop {
            if let Some(update) = ui_state_receiver.recv().await {
                match update {
                    StateUpdate::SetDevices(devices) => main_window.set_devices(&devices),
                    StateUpdate::SetLoading(is_loading) => main_window.set_loading(is_loading),
                    StateUpdate::SetDeviceState(state) => main_window.set_device_state(&state),
                    StateUpdate::SetEqualizerConfiguration(equalizer_configuration) => main_window.set_equalizer_configuration(equalizer_configuration),
                    StateUpdate::SetSelectedDevice(device) => main_window.set_property("selected-device", device),
                    StateUpdate::SetCustomEqualizerProfiles(custom_profiles) => main_window.set_custom_profiles(custom_profiles),
                    StateUpdate::AddToast(text) => main_window.add_toast(Toast::builder().title(&text).timeout(15).build()),
                }
            }
        }
    }));

    fn handle_error<T>(err: anyhow::Error, state: &State<T>)
    where
        T: DeviceRegistry,
    {
        let deselect_device = || {
            state
                .state_update_sender
                .send(StateUpdate::SetSelectedDevice(None))
                .unwrap()
        };

        let send_toast = |text| {
            state
                .state_update_sender
                .send(StateUpdate::AddToast(text))
                .unwrap()
        };

        tracing::error!("{err:?}");
        match err.downcast_ref::<openscq30_lib::Error>() {
            Some(openscq30_lib::Error::NotConnected { .. }) => {
                deselect_device();
                send_toast("Device Disconnected".to_string());
            }
            Some(openscq30_lib::Error::DeviceNotFound { .. }) => {
                deselect_device();
                send_toast("Device Not Found".to_string());
            }
            Some(openscq30_lib::Error::ServiceNotFound { .. }) => {
                deselect_device();
                send_toast("Device BLE Service Not Found".to_string());
            }
            Some(openscq30_lib::Error::CharacteristicNotFound { .. }) => {
                deselect_device();
                send_toast("Device BLE Characteristic Not Found".to_string());
            }
            Some(openscq30_lib::Error::NameNotFound { .. }) => {
                deselect_device();
                send_toast("Device Name Not Found".to_string());
            }
            Some(openscq30_lib::Error::NoResponse { .. }) => {
                deselect_device();
                send_toast("Device Didn't Respond".to_string());
            }
            Some(openscq30_lib::Error::FeatureNotSupported { feature_name }) => send_toast(
                format!("Tried to use feature not supported by device: {feature_name}"),
            ),
            Some(openscq30_lib::Error::WriteFailed { source }) => {
                send_toast(format!("Write to characteristic failed: {source:?}"))
            }
            Some(openscq30_lib::Error::IncompleteStateError { message }) => {
                send_toast(format!("Action failed due to incomplete state: {message}"))
            }
            Some(openscq30_lib::Error::Other { .. }) | None => {
                state
                    .state_update_sender
                    .send(StateUpdate::AddToast(format!("{err:#}")))
                    .unwrap();
            }
        }
    }

    let (action_sender, action_receiver) = MainContext::channel::<Action>(Priority::default());
    main_window.set_sender(action_sender);
    action_receiver.attach(
        None,
        clone!(@strong state, @strong settings => @default-return Continue(false), move |action: Action| {
            MainContext::default().spawn_local(clone!(@strong state, @strong settings => async move {
                let result = match action {
                    Action::SetAmbientSoundMode(ambient_sound_mode) => {
                        actions::set_ambient_sound_mode(&state, ambient_sound_mode)
                        .await
                        .context("ambient sound mode selected")
                    },
                    Action::SetNoiseCancelingMode(noise_canceling_mode) => {
                        actions::set_noise_canceling_mode(&state, noise_canceling_mode)
                        .await
                        .context("noise canceling mode selected")
                    },
                    Action::SetTransparencyMode(transparency_mode) => {
                        actions::set_transparency_mode(&state, transparency_mode)
                        .await
                        .context("transparency mode selected")
                    },
                    Action::SetCustomNoiseCanceling(custom_noise_canceling) => {
                        actions::set_custom_noise_canceling(&state, custom_noise_canceling)
                        .await
                        .context("transparency mode selected")
                    },
                    Action::Connect(mac_address) => {
                        actions::set_device(&state, Some(mac_address))
                        .await
                        .context("select device")
                    },
                    Action::Disconnect => {
                        actions::set_device(&state, None)
                        .await
                        .context("select device")
                    },
                    Action::SelectCustomEqualizerProfile(profile) => {
                        actions::select_custom_equalizer_configuration(&state, &settings.config, &profile)
                        .await
                        .context("custom equalizer profile selected")
                    },
                    Action::CreateCustomEqualizerProfile(profile) => {
                        actions::create_custom_equalizer_profile(&state, &settings.config, &profile)
                            .context("create custom equalizer profile")
                    },
                    Action::DeleteCustomEqualizerProfile(profile) => {
                        actions::delete_custom_equalizer_profile(&state, &settings.config, &profile)
                        .context("delete custom equalizer profile")
                    },
                    Action::SetEqualizerConfiguration(configuration) => {
                        actions::set_equalizer_configuration(&state, configuration)
                        .await
                        .context("apply equalizer settings")
                    },
                };
                if let Err(err) = result {
                    handle_error(err, &state);
                }
            }));
            Continue(true)
        }),
    );

    main_context.spawn_local(clone!(@weak main_window, @strong state => async move {
        loop {
            let next_state = state.state_update_receiver.next().await;
            main_window.set_device_state(&next_state);
        }
    }));

    let action_refresh_devices = SimpleAction::new("refresh-devices", None);
    action_refresh_devices.connect_activate(clone!(@strong state => move |_, _| {
        MainContext::default().spawn_local(clone!(@strong state => async move {
            actions::refresh_devices(&state)
                .await
                .context("refresh devices")
                .unwrap_or_else(|err| handle_error(err, &state));
        }));
    }));
    main_window.add_action(&action_refresh_devices);
    application.set_accels_for_action("win.refresh-devices", &["<Ctrl>R", "F5"]);

    action_refresh_devices.activate(None);
}

#[cfg(target_os = "windows")]
fn set_ui_theme(application: &adw::Application) -> anyhow::Result<()> {
    let settings = UISettings::new()?;

    use adw::prelude::AdwApplicationExt;
    use std::sync::Arc;
    use tokio::sync::Notify;

    let notify_colors_changed = Arc::new(Notify::new());
    {
        let notify_colors_changed = notify_colors_changed.to_owned();
        settings.ColorValuesChanged(&TypedEventHandler::new(
            move |_settings: &Option<UISettings>, _: &Option<IInspectable>| {
                notify_colors_changed.notify_one();
                Ok(())
            },
        ))?;
    }

    let style_manager = application.style_manager();
    MainContext::default().spawn_local(async move {
        loop {
            // Initially set color scheme, then wait for changes
            match settings.GetColorValue(UIColorType::Foreground) {
                Ok(color) => {
                    let color_scheme = if is_color_light(&color) {
                        adw::ColorScheme::PreferLight
                    } else {
                        adw::ColorScheme::PreferDark
                    };
                    style_manager.set_color_scheme(color_scheme);
                }
                Err(err) => tracing::warn!("failed to set color scheme: {err:?}"),
            }

            notify_colors_changed.notified().await;
            tracing::info!("updating ui color scheme");
        }
    });
    Ok(())
}

#[cfg(target_os = "windows")]
fn is_color_light(color: &windows::UI::Color) -> bool {
    // https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/apply-windows-themes
    let lhs = (5 * color.G as u32) + (2 * color.R as u32) + color.B as u32;
    let rhs = 8 * 128;
    lhs < rhs
}

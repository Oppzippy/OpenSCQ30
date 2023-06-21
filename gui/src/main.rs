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
    gio::{self, SimpleAction},
    glib::{self, clone, closure_local, MainContext, OptionFlags},
    prelude::*,
    traits::GtkWindowExt,
};
use gtk_openscq30_lib::GtkDeviceRegistry;
use logging_level::LoggingLevel;
use openscq30_lib::api::device::DeviceRegistry;
use settings::Settings;
use tracing::Level;
use widgets::MainWindow;
#[cfg(target_os = "windows")]
use windows::{
    core::IInspectable,
    Foundation::TypedEventHandler,
    UI::ViewManagement::{UIColorType, UISettings},
};

use crate::objects::CustomEqualizerProfileObject;

mod actions;
mod gettext;
mod gettext_sys;
mod gtk_openscq30_lib;
mod logging_level;
#[cfg(test)]
mod mock;
mod objects;
mod settings;
mod swappable_broadcast;
#[allow(clippy::new_without_default)]
mod widgets;

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
    app.connect_activate(build_ui);
    handle_command_line_args(&app);

    app.run();
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
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap_or_else(|err| panic!("failed to start tokio runtime: {err}"));

    let registry = tokio_runtime
        .block_on(openscq30_lib::api::new_soundcore_device_registry())
        .unwrap_or_else(|err| panic!("failed to initialize device registry: {err}"));

    let gtk_registry = GtkDeviceRegistry::new(registry, tokio_runtime);
    build_ui_2(application, gtk_registry)
}

fn build_ui_2(
    application: &adw::Application,
    gtk_registry: GtkDeviceRegistry<impl DeviceRegistry + Send + Sync + 'static>,
) {
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
        .get(|settings| {
            main_window.set_custom_profiles(
                settings
                    .custom_profiles()
                    .iter()
                    .map(|(name, profile)| {
                        CustomEqualizerProfileObject::new(name, profile.volume_adjustments())
                    })
                    .collect(),
            );
        })
        .unwrap();

    let (state, mut ui_state_receiver) = State::new(gtk_registry);
    let state = Rc::new(state);

    let main_context = MainContext::default();
    main_context.spawn_local(clone!(@weak main_window => async move {
        loop {
            if let Some(update) = ui_state_receiver.recv().await {
                match update {
                    StateUpdate::SetDevices(devices) => main_window.set_devices(&devices),
                    StateUpdate::SetLoading(is_loading) => main_window.set_loading(is_loading),
                    StateUpdate::SetAmbientSoundMode(ambient_sound_mode) => main_window.set_ambient_sound_mode(ambient_sound_mode),
                    StateUpdate::SetNoiseCancelingMode(noise_canceling_mode) => main_window.set_noise_canceling_mode(noise_canceling_mode),
                    StateUpdate::SetEqualizerConfiguration(equalizer_configuration) => main_window.set_equalizer_configuration(&equalizer_configuration),
                    StateUpdate::SetSelectedDevice(device) => main_window.set_property("selected-device", device),
                    StateUpdate::SetCustomEqualizerProfiles(custom_profiles) => main_window.set_custom_profiles(custom_profiles),
                    StateUpdate::AddToast(text) => main_window.add_toast(Toast::builder().title(&text).timeout(15).build()),
                }
            }
        }
    }));

    fn handle_error<T>(err: anyhow::Error, state: &State<T>)
    where
        T: DeviceRegistry + Send + Sync,
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
            Some(openscq30_lib::Error::Other { .. }) | None => {
                state
                    .state_update_sender
                    .send(StateUpdate::AddToast(format!("{err:#}")))
                    .unwrap();
            }
        }
    }

    main_context.spawn_local(clone!(@weak main_window, @strong state => async move {
        loop {
            let next_state = state.state_update_receiver.next().await;
            main_window.set_ambient_sound_mode(next_state.ambient_sound_mode());
            main_window.set_noise_canceling_mode(next_state.noise_canceling_mode());
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

    main_context.spawn_local(clone!(@strong action_refresh_devices => async move {
        action_refresh_devices.activate(None);
    }));

    main_window.connect_notify_local(
        Some("selected-device"),
        clone!(@strong state => move |main_window, _| {
            MainContext::default().spawn_local(clone!(@strong state, @weak main_window => async move {
                actions::set_device(
                    &state,
                    main_window.selected_device(),
                )
                .await
                .context("select device")
                .unwrap_or_else(|err| handle_error(err, &state));
            }));
        }),
    );

    main_window.connect_closure(
        "ambient-sound-mode-selected",
        false,
        closure_local!(@strong state => move |_main_window: MainWindow, ambient_sound_mode_id: u8| {
            MainContext::default().spawn_local(clone!(@strong state => async move {
                actions::set_ambient_sound_mode(&state, ambient_sound_mode_id)
                .await
                .context("ambient sound mode selected")
                .unwrap_or_else(|err| handle_error(err, &state));
            }));
        }),
    );

    main_window.connect_closure(
        "noise-canceling-mode-selected",
        false,
        closure_local!(@strong state => move |_main_window: MainWindow, noise_canceling_mode_id: u8| {
            MainContext::default().spawn_local(clone!(@strong state => async move {
                actions::set_noise_canceling_mode(&state, noise_canceling_mode_id)
                .await
                .context("noise canceling mode selected")
                .unwrap_or_else(|err| handle_error(err, &state));
            }));
        }),
    );

    main_window.connect_closure(
        "apply-equalizer-settings",
        false,
        closure_local!(@strong state => move |main_window: MainWindow| {
            MainContext::default().spawn_local(clone!(@strong state => async move {
                actions::set_equalizer_configuration(&state, main_window.equalizer_configuration())
                .await
                .context("apply equalizer settings")
                .unwrap_or_else(|err| handle_error(err, &state));
            }));
        }),
    );

    main_window.connect_closure(
        "custom-equalizer-profile-selected",
        false,
        closure_local!(@strong state, @strong settings => move |_main_window: MainWindow, custom_profile: &CustomEqualizerProfileObject| {
            actions::select_custom_equalizer_configuration(&state, &settings.config, custom_profile)
                .context("custom equalizer profile selected")
                .unwrap_or_else(|err| handle_error(err, &state));
        }),
    );

    main_window.connect_closure(
        "create-custom-equalizer-profile",
        false,
        closure_local!(@strong state, @strong settings => move |_main_window: MainWindow, custom_profile: &CustomEqualizerProfileObject| {
            actions::create_custom_equalizer_profile(&state, &settings.config, custom_profile)
                .context("create custom equalizer profile")
                .unwrap_or_else(|err| handle_error(err, &state));
        }),
    );

    main_window.connect_closure(
        "delete-custom-equalizer-profile",
        false,
        closure_local!(@strong state, @strong settings => move |_main_window: MainWindow, custom_profile: &CustomEqualizerProfileObject| {
            actions::delete_custom_equalizer_profile(&state, &settings.config, custom_profile)
                .context("delete custom equalizer profile")
                .unwrap_or_else(|err| handle_error(err, &state));
        }),
    );

    main_window.present();
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

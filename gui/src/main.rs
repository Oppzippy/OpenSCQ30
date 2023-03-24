use std::{rc::Rc, str::FromStr, sync::Once};

use actions::{State, StateUpdate};
use gtk::{
    gio::{self, SimpleAction},
    glib::{self, clone, closure_local, MainContext, OptionFlags},
    prelude::*,
    traits::GtkWindowExt,
    Application,
};
use gtk_openscq30_lib::GtkDeviceRegistry;
use logging_level::LoggingLevel;
use openscq30_lib::api::device::DeviceRegistry;
use settings::Settings;
use tracing::Level;
use widgets::MainWindow;

use crate::objects::EqualizerCustomProfileObject;

mod actions;
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
    load_resources();
    run_application();
}

static LOAD_RESOURCES: Once = Once::new();

pub fn load_resources() {
    LOAD_RESOURCES.call_once(|| {
        gio::resources_register_include!("widgets.gresource").expect("failed to load widgets");
    });
}

#[cfg(not(feature = "libadwaita"))]
fn run_application() {
    let app = gtk::Application::builder()
        .application_id(APPLICATION_ID_STR)
        .build();
    app.connect_activate(build_ui);
    handle_command_line_args(&app);

    app.run();
}

#[cfg(feature = "libadwaita")]
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
        glib::char::Char('l' as i8),
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

fn build_ui(application: &impl IsA<Application>) {
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
    application: &impl IsA<Application>,
    gtk_registry: GtkDeviceRegistry<impl DeviceRegistry + Send + Sync + 'static>,
) {
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
                        EqualizerCustomProfileObject::new(name, profile.volume_offsets())
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
                    StateUpdate::SetEqualizerCustomProfiles(custom_profiles) => main_window.set_custom_profiles(custom_profiles),
                }
            }
        }
    }));

    main_context.spawn_local(clone!(@weak main_window, @strong state => async move {
        loop {
            let next_state = state.state_update_receiver.next().await;
            main_window.set_ambient_sound_mode(next_state.ambient_sound_mode());
            main_window.set_noise_canceling_mode(next_state.noise_canceling_mode());
        }
    }));

    let action_refresh_devices = SimpleAction::new("refresh-devices", None);
    action_refresh_devices.connect_activate(clone!(@strong state => move |_, _| {
        actions::refresh_devices(&state);
    }));
    main_window.add_action(&action_refresh_devices);
    application.set_accels_for_action("win.refresh-devices", &["<Ctrl>R", "F5"]);

    main_context.spawn_local(clone!(@strong action_refresh_devices => async move {
        action_refresh_devices.activate(None);
    }));

    main_window.connect_notify_local(
        Some("selected-device"),
        clone!(@strong state => move |main_window, _| {
            actions::set_device(
                &state,
                main_window.selected_device(),
            );
        }),
    );

    main_window.connect_closure(
        "ambient-sound-mode-selected",
        false,
        closure_local!(@strong state => move |_main_window: MainWindow, ambient_sound_mode_id: u8| {
            actions::set_ambient_sound_mode(&state, ambient_sound_mode_id);
        }),
    );

    main_window.connect_closure(
        "noise-canceling-mode-selected",
        false,
        closure_local!(@strong state => move |_main_window: MainWindow, noise_canceling_mode_id: u8| {
            actions::set_noise_canceling_mode(&state, noise_canceling_mode_id);
        }),
    );

    main_window.connect_closure(
        "apply-equalizer-settings",
        false,
        closure_local!(@strong state => move |main_window: MainWindow| {
            actions::set_equalizer_configuration(&state, main_window.equalizer_configuration());
        }),
    );

    main_window.connect_closure(
        "custom-equalizer-profile-selected",
        false,
        closure_local!(@strong state, @strong settings => move |_main_window: MainWindow, custom_profile: &EqualizerCustomProfileObject| {
            actions::select_custom_equalizer_configuration(&state, &settings.config, custom_profile);
        }),
    );

    main_window.connect_closure(
        "create-custom-equalizer-profile",
        false,
        closure_local!(@strong state, @strong settings => move |_main_window: MainWindow, custom_profile: &EqualizerCustomProfileObject| {
            actions::create_custom_equalizer_profile(&state, &settings.config, custom_profile);
        }),
    );

    main_window.connect_closure(
        "delete-custom-equalizer-profile",
        false,
        closure_local!(@strong state, @strong settings => move |_main_window: MainWindow, custom_profile: &EqualizerCustomProfileObject| {
            actions::delete_custom_equalizer_profile(&state, &settings.config, custom_profile);
        }),
    );

    main_window.present();
}

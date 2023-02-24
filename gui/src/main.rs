use std::{cell::RefCell, rc::Rc, sync::Arc};

use gtk::{
    gio::{self, SimpleAction},
    glib::{self, clone, closure_local, MainContext, JoinHandle},
    prelude::*,
    traits::GtkWindowExt,
    Application,
};
use gtk_openscq30_lib::GtkDeviceRegistry;
use openscq30_lib::{
    api::device::{DeviceDescriptor, DeviceRegistry},
    packets::structures::{
        AmbientSoundMode, EqualizerBandOffsets, EqualizerConfiguration, NoiseCancelingMode,
    },
    state::DeviceState,
};
use settings::{EqualizerCustomProfile, SettingsFile};
use swappable_broadcast::SwappableBroadcastReceiver;
use tracing::Level;
#[cfg(debug_assertions)]
use tracing_subscriber::fmt::format::FmtSpan;
use widgets::{Device, MainWindow};

use crate::objects::{DeviceObject, EqualizerCustomProfileObject};

mod gtk_openscq30_lib;
mod objects;
mod settings;
mod swappable_broadcast;
mod widgets;

fn main() {
    let subscriber_builder = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .pretty();
    #[cfg(debug_assertions)]
    let subscriber_builder = subscriber_builder
        .with_max_level(Level::TRACE)
        .with_span_events(FmtSpan::ACTIVE);
    #[cfg(not(debug_assertions))]
    let subscriber_builder = subscriber_builder.with_max_level(Level::INFO);
    subscriber_builder.init();

    load_resources();
    run_application();
}

#[cfg(not(feature = "libadwaita"))]
fn run_application() {
    let app = gtk::Application::builder()
        .application_id("com.oppzippy.OpenSCQ30")
        .build();
    app.connect_activate(build_ui);

    app.run();
}

#[cfg(feature = "libadwaita")]
fn run_application() {
    let app = adw::Application::builder()
        .application_id("com.oppzippy.OpenSCQ30")
        .build();
    app.connect_activate(build_ui);

    app.run();
}

fn get_settings_file() -> SettingsFile {
    let settings = SettingsFile::new(
        glib::user_data_dir()
            .join("OpenSCQ30")
            .join("settings.toml"),
    );
    if let Err(err) = settings.load() {
        tracing::warn!("initial load of settings file failed: {:?}", err)
    }
    settings
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

    let gtk_registry = Arc::new(GtkDeviceRegistry::new(registry, tokio_runtime));
    build_ui_2(application, gtk_registry)
}

fn build_ui_2(
    application: &impl IsA<Application>,
    gtk_registry: Arc<GtkDeviceRegistry<impl DeviceRegistry + Send + Sync + 'static>>,
) {
    let settings_file = Rc::new(get_settings_file());
    let main_window = MainWindow::new(application, settings_file.to_owned());
    settings_file
        .get(|settings| {
            main_window.set_custom_profiles(
                settings
                    .equalizer_custom_profiles
                    .iter()
                    .map(|(name, profile)| {
                        EqualizerCustomProfileObject::new(name, profile.volume_offsets)
                    })
                    .collect(),
            );
        })
        .unwrap();

    let state_update_receiver: Rc<SwappableBroadcastReceiver<DeviceState>> =
        Rc::new(SwappableBroadcastReceiver::new());
    let selected_device = Rc::new(RefCell::new(None));

    let main_context = MainContext::default();
    main_context.spawn_local(
        clone!(@weak main_window, @strong state_update_receiver => async move {
            loop {
                let next_state = state_update_receiver.next().await;
                main_window.set_ambient_sound_mode(next_state.ambient_sound_mode());
                main_window.set_noise_canceling_mode(next_state.noise_canceling_mode());
            }
        }),
    );

    let action_refresh_devices = SimpleAction::new("refresh-devices", None);
    action_refresh_devices.connect_activate(clone!(@weak main_window, @strong gtk_registry, @strong selected_device, @strong state_update_receiver => move |_, _| {
        let main_context = MainContext::default();
        main_context.spawn_local(
            clone!(@weak main_window, @strong gtk_registry, @strong selected_device, @strong state_update_receiver => async move {
                match gtk_registry
                    .device_descriptors()
                    .await {
                    Ok(descriptors) => {
                        let model_devices = descriptors
                            .iter()
                            .map(|descriptor| Device { mac_address: descriptor.mac_address().to_owned(), name: descriptor.name().to_owned() })
                            .collect::<Vec<_>>();
                        if model_devices.is_empty() {
                            state_update_receiver.replace_receiver(None).await;
                            *selected_device.borrow_mut() = None;
                        }
                        main_window.set_devices(&model_devices);
                    },
                    Err(err) => {
                        tracing::warn!("error obtaining device descriptors: {err}")
                    },
                }
            }),
        );
    }));
    main_window.add_action(&action_refresh_devices);
    application.set_accels_for_action("win.refresh-devices", &["<Ctrl>R", "F5"]);

    main_context.spawn_local(clone!(@strong action_refresh_devices => async move {
        action_refresh_devices.activate(None);
    }));
    
    let connect_to_device_handle: Arc<RefCell<Option<JoinHandle<()>>>> = Arc::new(RefCell::new(None));

    main_window.connect_notify_local(
        Some("selected-device"),
        clone!(@strong state_update_receiver, @strong gtk_registry, @strong selected_device, @strong connect_to_device_handle => move |main_window, _| {
            // Clean up any existing devices
            if let Some(handle) = &*connect_to_device_handle.borrow_mut() {
                handle.abort();
            }
            *selected_device.borrow_mut() = None;
            main_window.set_loading(false);
            
            // Connect to new device
            if let Some(new_selected_device) = main_window.selected_device() {
                main_window.set_loading(true);
                let main_context = MainContext::default();
                *connect_to_device_handle.borrow_mut() = Some(
                    main_context.spawn_local(
                        clone!(@weak main_window, @strong gtk_registry, @strong selected_device, @strong state_update_receiver => async move {
                            match gtk_registry.device(new_selected_device.mac_address()).await {
                                Ok(Some(device)) => {
                                    *selected_device.borrow_mut() = Some(device.to_owned());
                                    let receiver = device.subscribe_to_state_updates();
                                    state_update_receiver.replace_receiver(Some(receiver)).await;

                                    let ambient_sound_mode = device.ambient_sound_mode().await;
                                    let noise_canceling_mode = device.noise_canceling_mode().await;
                                    let equalizer_configuration = device.equalizer_configuration().await;

                                    main_window.set_ambient_sound_mode(ambient_sound_mode);
                                    main_window.set_noise_canceling_mode(noise_canceling_mode);
                                    main_window.set_equalizer_configuration(&equalizer_configuration);
                                },
                                Ok(None) => {
                                    tracing::warn!("could not find selected device: {:?}", new_selected_device);
                                },
                                Err(err) => { 
                                    tracing::warn!("error connecting to device {:?}: {err}", new_selected_device);
                                },
                            }
                            
                            main_window.set_loading(false);
                            if selected_device.borrow().is_none() {
                                main_window.set_property("selected-device", None as Option<DeviceObject>);
                            }
                        })
                    )
                );
            }
        }),
    );

    main_window.connect_closure(
        "ambient-sound-mode-selected",
        false,
        closure_local!(@strong selected_device => move |_main_window: MainWindow, mode_id: u8| {
            let main_context = MainContext::default();
            main_context.spawn_local(clone!(@strong selected_device => async move {
                let Some(device) = &*selected_device.borrow() else {
                    tracing::warn!("no device is selected");
                    return;
                };
                let Some(ambient_sound_mode) = AmbientSoundMode::from_id(mode_id) else {
                    tracing::warn!("invalid ambient sound mode: {mode_id}");
                    return;
                };
                if let Err(err) = device.set_ambient_sound_mode(ambient_sound_mode).await {
                    tracing::error!("error setting ambient sound mode: {err}")
                }
            }));
        }),
    );

    main_window.connect_closure(
        "noise-canceling-mode-selected",
        false,
        closure_local!(@strong selected_device => move |_main_window: MainWindow, mode_id: u8| {
            let main_context = MainContext::default();
            main_context.spawn_local(
                clone!(@strong selected_device => async move {
                    let Some(device) = &*selected_device.borrow() else {
                        tracing::warn!("no device is selected");
                        return;
                    };
                    let Some(noise_canceling_mode) = NoiseCancelingMode::from_id(mode_id) else {
                        tracing::error!("invalid noise canceling mode: {mode_id}");
                        return;
                    };
                    if let Err(err) = device.set_noise_canceling_mode(noise_canceling_mode).await {
                        tracing::error!("error setting noise canceling mode: {err}")
                    }
                }),
            );
        }),
    );

    main_window.connect_closure(
        "apply-equalizer-settings",
        false,
        closure_local!(@strong selected_device => move |main_window: MainWindow| {
            let main_context = MainContext::default();
            main_context.spawn_local(
                clone!(@strong selected_device => async move {
                    let Some(device) = &*selected_device.borrow() else {
                        tracing::warn!("no device is selected");
                        return;
                    };
                    let configuration = main_window.equalizer_configuration();
                    if let Err(err) = device.set_equalizer_configuration(configuration).await {
                        tracing::error!("error setting equalizer configuration: {err}");
                    }
                }),
            );
        }),
    );

    main_window.connect_closure(
        "custom-equalizer-profile-selected",
        false,
        closure_local!(@strong settings_file => move |main_window: MainWindow, custom_profile: &EqualizerCustomProfileObject| {
            let result = settings_file.get(|settings| {
                match settings.equalizer_custom_profiles.get(&custom_profile.name()) {
                    Some(profile) => {
                        main_window.set_equalizer_configuration(
                            &EqualizerConfiguration::new_custom_profile(EqualizerBandOffsets::new(profile.volume_offsets))
                        );
                    },
                    None => {
                        tracing::warn!("custom profile does not exist: {}", custom_profile.name());
                    },
                }
            });
            if let Err(err) = result {
                tracing::warn!("unable to get settings file: {:?}", err);
            }
        }),
    );

    main_window.connect_closure(
        "create-custom-equalizer-profile",
        false,
        closure_local!(@strong settings_file => move |main_window: MainWindow, custom_profile: &EqualizerCustomProfileObject| {
            settings_file.edit(|settings| {
                settings.equalizer_custom_profiles.insert(
                    custom_profile.name(),
                    EqualizerCustomProfile {
                        volume_offsets: custom_profile.volume_offsets()
                    }
                );
            }).unwrap();
            settings_file.get(|settings| {
                main_window.set_custom_profiles(
                    settings.equalizer_custom_profiles
                        .iter()
                        .map(|(name, profile)| EqualizerCustomProfileObject::new(name, profile.volume_offsets))
                        .collect()
                );
            }).unwrap();
        }),
    );

    main_window.connect_closure(
        "delete-custom-equalizer-profile",
        false,
        closure_local!(@strong settings_file => move |main_window: MainWindow, custom_profile: &EqualizerCustomProfileObject| {
            settings_file.edit(|settings| {
                settings.equalizer_custom_profiles.remove(&custom_profile.name());
            }).unwrap();
            settings_file.get(|settings| {
                main_window.set_custom_profiles(
                    settings.equalizer_custom_profiles
                        .iter()
                        .map(|(name, profile)| EqualizerCustomProfileObject::new(name, profile.volume_offsets))
                        .collect()
                );
            }).unwrap();
        }),
    );

    main_window.present();
}

fn load_resources() {
    gio::resources_register_include!("widgets.gresource").expect("failed to load widgets");
}

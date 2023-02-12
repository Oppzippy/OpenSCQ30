use std::{sync::Arc, rc::Rc};

use gtk::{
    gio,
    glib::{self, clone, closure_local, MainContext},
    prelude::{ApplicationExt, ApplicationExtManual, ObjectExt, IsA},
    traits::GtkWindowExt,
    Application,
};
use gtk_openscq30_lib::GtkSoundcoreDeviceRegistry;
use openscq30_lib::{
    api::SoundcoreDeviceRegistry,
    packets::structures::{
        AmbientSoundMode, NoiseCancelingMode, EqualizerConfiguration, EqualizerBandOffsets,
    }, soundcore_bluetooth, state::SoundcoreDeviceState,
};
use settings::{SettingsFile, EqualizerCustomProfile};
use swappable_broadcast::SwappableBroadcastReceiver;
use tracing::Level;
#[cfg(debug_assertions)]
use tracing_subscriber::fmt::format::FmtSpan;
use widgets::{MainWindow, Device};

use crate::objects::EqualizerCustomProfileObject;

mod objects;
mod widgets;
mod gtk_openscq30_lib;
mod swappable_broadcast;
mod settings;

fn main() {
    let subscriber_builder = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .pretty();
    #[cfg(debug_assertions)]
    let subscriber_builder = subscriber_builder.with_max_level(Level::TRACE).with_span_events(FmtSpan::ACTIVE);
    #[cfg(not(debug_assertions))]
    let subscriber_builder = subscriber_builder.with_max_level(Level::INFO);
    subscriber_builder.init();

    load_resources();
    run_application();
}

#[cfg(not(feature = "libadwaita"))]
fn run_application() {
    let app = gtk::Application::builder()
        .application_id("com.oppzippy.openscq30")
        .build();
    app.connect_activate(build_ui);

    app.run();
}

#[cfg(feature = "libadwaita")]
fn run_application() {
    let app = adw::Application::builder()
        .application_id("com.oppzippy.openscq30")
        .build();
    app.connect_activate(build_ui);

    app.run();
}

fn get_settings_file() -> SettingsFile {
    let settings = SettingsFile::new(glib::user_data_dir().join("OpenSCQ30").join("settings.toml"));
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

    let connection_registry_impl = tokio_runtime.block_on(soundcore_bluetooth::new_connection_registry())
        .unwrap_or_else(|err| panic!("failed to initialize handler: {err}"));
    let registry = tokio_runtime.block_on(SoundcoreDeviceRegistry::new(connection_registry_impl))
        .unwrap_or_else(|err| panic!("failed to initialize device registry: {err}"));

    if let Err(err) = tokio_runtime.block_on(registry.refresh_devices()) {
        tracing::error!("error on initial refresh_devices: {}", err);
    }

    let gtk_registry = Arc::new(GtkSoundcoreDeviceRegistry::new(registry, tokio_runtime));

    let settings_file = Rc::new(get_settings_file());
    // settings_file.edit(|settings| {
    //     settings.equalizer_custom_profiles.insert("asdf".to_string(), EqualizerCustomProfile {
    //         volume_offsets: [0,1,2,3,4,5,6,7],
    //     });
    //     settings.equalizer_custom_profiles.insert("qwer".to_string(), EqualizerCustomProfile {
    //         volume_offsets: [-60,1,2,3,4,5,6,7],
    //     });
    // }).unwrap();
    let main_window = MainWindow::new(application, settings_file.to_owned());
    settings_file.get(|settings| {
        main_window.set_custom_profiles(
            settings.equalizer_custom_profiles
                .iter()
                .map(|(name, profile)| EqualizerCustomProfileObject::new(name, profile.volume_offsets))
                .collect()
        );
    }).unwrap();

    let state_update_receiver: Arc<SwappableBroadcastReceiver<SoundcoreDeviceState>> = Arc::new(SwappableBroadcastReceiver::new());

    let main_context = MainContext::default();
    let gtk_registry_clone = gtk_registry.clone();
    main_context.spawn_local(clone!(@weak main_window => async move {
        let bluetooth_devices = gtk_registry_clone
            .devices()
            .await;
        let mut model_devices = Vec::new();
        for bluetooth_device in bluetooth_devices {
            model_devices.push(Device {
                mac_address: bluetooth_device.mac_address().await.unwrap_or_else(|_| "Unknown MAC Address".to_string()),
                name: bluetooth_device.name().await.unwrap_or_else(|_| "Unknown Name".to_string()),
            })
        }
        main_window.set_devices(&model_devices);
    }));
    
    main_context.spawn_local(clone!(@weak main_window, @strong state_update_receiver => async move {
        loop {
            let next_state = state_update_receiver.next().await;
            main_window.set_ambient_sound_mode(next_state.ambient_sound_mode());
            main_window.set_noise_canceling_mode(next_state.noise_canceling_mode());               
        }
    }));

    let gtk_registry_clone = gtk_registry.clone();
    main_window.connect_closure(
        "refresh-devices",
        false,
        closure_local!(move |main_window: MainWindow| {
            let main_context = MainContext::default();
            main_context.spawn_local(clone!(@weak main_window, @weak gtk_registry_clone => async move {
                if let Err(err) = gtk_registry_clone.refresh_devices().await {
                    tracing::warn!("error refreshing devices: {err}");
                    return
                };

                let bluetooth_devices = gtk_registry_clone
                    .devices()
                    .await;
                let mut model_devices = Vec::new();
                for bluetooth_device in bluetooth_devices {
                    model_devices.push(Device {
                        mac_address: bluetooth_device.mac_address().await.unwrap_or_else(|_| "Unknown MAC Address".to_string()),
                        name: bluetooth_device.name().await.unwrap_or_else(|_| "Unknown Name".to_string()),
                    })
                }
                main_window.set_devices(&model_devices);
            }));
        }),
    );

    let gtk_registry_clone = gtk_registry.clone();
    main_window.connect_closure(
        "device_selection_changed",
        false,
        closure_local!(@weak-allow-none state_update_receiver => move |main_window: MainWindow| {
            if let Some(selected_device) = main_window.selected_device() {
                let main_context = MainContext::default();
                main_context.spawn_local(clone!(@strong main_window, @weak gtk_registry_clone => async move {
                    match gtk_registry_clone.device_by_mac_address(&selected_device.mac_address).await {
                        Some(device) => {
                            let receiver = device.subscribe_to_state_updates();
                            state_update_receiver.unwrap().replace_receiver(receiver).await;

                            let ambient_sound_mode = device.ambient_sound_mode().await;
                            let noise_canceling_mode = device.noise_canceling_mode().await;
                            let equalizer_configuration = device.equalizer_configuration().await;
                            
                            main_window.set_ambient_sound_mode(ambient_sound_mode);
                            main_window.set_noise_canceling_mode(noise_canceling_mode);
                            main_window.set_equalizer_configuration(&equalizer_configuration);
                        },
                        None => tracing::warn!("could not find selected device: {}", selected_device.mac_address),
                    }
                }));
            }
        }),
    );

    let gtk_registry_clone = gtk_registry.clone();
    main_window.connect_closure(
        "ambient-sound-mode-selected",
        false,
        closure_local!(move |main_window: MainWindow, mode_id: u8| {
            let main_context = MainContext::default();
            main_context.spawn_local(clone!(@weak main_window, @weak gtk_registry_clone => async move {
                let Some(selected_device) = main_window.selected_device() else {
                    tracing::warn!("no device is selected");
                    return;
                };
                let Some(device) = gtk_registry_clone.device_by_mac_address(&selected_device.mac_address).await else {
                    tracing::warn!("could not find selected device: {}", selected_device.mac_address);
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

    let gtk_registry_clone = gtk_registry.clone();
    main_window.connect_closure(
        "noise-canceling-mode-selected",
        false,
        closure_local!(move |main_window: MainWindow, mode_id: u8| {
            let main_context = MainContext::default();
            main_context.spawn_local(clone!(@weak main_window, @weak gtk_registry_clone => async move {
                let Some(selected_device) = main_window.selected_device() else {
                    tracing::warn!("no device is selected");
                    return;
                };
                let Some(device) = gtk_registry_clone.device_by_mac_address(&selected_device.mac_address).await else {
                    tracing::warn!("could not find selected device: {}", selected_device.mac_address);
                    return;
                };
                let Some(noise_canceling_mode) = NoiseCancelingMode::from_id(mode_id) else {
                    tracing::error!("invalid noise canceling mode: {mode_id}");
                    return;
                };
                if let Err(err) = device.set_noise_canceling_mode(noise_canceling_mode).await {
                    tracing::error!("error setting noise canceling mode: {err}")                            
                }
            }));
        }),
    );

    let gtk_registry_clone = gtk_registry.clone();
    main_window.connect_closure(
        "apply-equalizer-settings",
        false,
        closure_local!(move |main_window: MainWindow| {
            let main_context = MainContext::default();
            main_context.spawn_local(clone!(@weak main_window, @weak gtk_registry_clone => async move {
                let Some(selected_device) = main_window.selected_device() else {
                    tracing::warn!("no device is selected");
                    return;
                };
                let Some(device) = gtk_registry_clone.device_by_mac_address(&selected_device.mac_address).await else {
                    tracing::warn!("could not find selected device: {}", selected_device.mac_address);
                    return;
                };
                let configuration = main_window.equalizer_configuration();
                if let Err(err) = device.set_equalizer_configuration(configuration).await {
                    tracing::error!("error setting equalizer configuration: {err}");                            
                }
            }));
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
    gio::resources_register_include!("widgets.gresource")
        .expect("failed to load widgets");
}

use std::sync::Arc;

use gtk::{
    gio,
    glib::{self, clone, closure_local, MainContext},
    prelude::{ApplicationExt, ApplicationExtManual, ObjectExt},
    traits::GtkWindowExt,
    ApplicationWindow,
};
use gtk_openscq30_lib::GtkSoundcoreDeviceRegistry;
use openscq30_lib::{
    api::SoundcoreDeviceRegistry,
    packets::structures::{
        AmbientSoundMode, NoiseCancelingMode,
    },
};
use tracing::Level;
use widgets::{MainWindow, Device};

mod objects;
mod widgets;
mod gtk_openscq30_lib;

fn main() {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(Level::TRACE)
        .init();

    load_resources();

    let app = adw::Application::builder()
        .application_id("com.oppzippy.openscq30")
        .build();
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &adw::Application) {
    let tokio_runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap_or_else(|err| panic!("failed to start tokio runtime: {err}"));

    let registry = tokio_runtime.block_on(SoundcoreDeviceRegistry::new())
        .unwrap_or_else(|err| panic!("failed to initialize device registry: {err}"));

    if let Err(err) = tokio_runtime.block_on(registry.refresh_devices()) {
        tracing::error!("error on initial refresh_devices: {}", err);
    }

    let gtk_registry = Arc::new(GtkSoundcoreDeviceRegistry::new(registry, tokio_runtime));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("OpenSCQ30")
        .build();

    let main_window = MainWindow::new();

    let main_context = MainContext::default();
    let gtk_registry_clone = gtk_registry.clone();
    main_context.spawn_local(clone!(@weak main_window => async move {
        let bluetooth_devices = gtk_registry_clone
            .get_devices()
            .await;
        let mut model_devices = Vec::new();
        for bluetooth_device in bluetooth_devices {
            model_devices.push(Device {
                mac_address: bluetooth_device.get_mac_address().await.unwrap_or_else(|_| "Unknown MAC Address".to_string()),
                name: bluetooth_device.get_name().await.unwrap_or_else(|_| "Unknown Name".to_string()),
            })
        }
        main_window.set_devices(&model_devices);
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
                    .get_devices()
                    .await;
                let mut model_devices = Vec::new();
                for bluetooth_device in bluetooth_devices {
                    model_devices.push(Device {
                        mac_address: bluetooth_device.get_mac_address().await.unwrap_or_else(|_| "Unknown MAC Address".to_string()),
                        name: bluetooth_device.get_name().await.unwrap_or_else(|_| "Unknown Name".to_string()),
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
        closure_local!(move |main_window: MainWindow| {
            if let Some(selected_device) = main_window.selected_device() {
                let main_context = MainContext::default();
                main_context.spawn_local(clone!(@weak main_window, @weak gtk_registry_clone => async move {
                    match gtk_registry_clone.get_device_by_mac_address(&selected_device.mac_address).await {
                        Some(device) => {
                            let ambient_sound_mode = device.get_ambient_sound_mode().await;
                            let noise_canceling_mode = device.get_noise_canceling_mode().await;
                            let equalizer_configuration = device.get_equalizer_configuration().await;
                            
                            main_window.set_ambient_sound_mode(ambient_sound_mode);
                            main_window.set_noise_canceling_mode(noise_canceling_mode);
                            main_window.set_equalizer_configuration(equalizer_configuration);
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
                let Some(device) = gtk_registry_clone.get_device_by_mac_address(&selected_device.mac_address).await else {
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
                let Some(device) = gtk_registry_clone.get_device_by_mac_address(&selected_device.mac_address).await else {
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
                let Some(device) = gtk_registry_clone.get_device_by_mac_address(&selected_device.mac_address).await else {
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

    let gtk_registry_clone = gtk_registry.clone();
    main_window.connect_closure(
        "refresh-equalizer-settings",
        false,
        closure_local!(move |main_window: MainWindow| {
            let main_context = MainContext::default();
            main_context.spawn_local(clone!(@weak main_window, @weak gtk_registry_clone => async move {
                let Some(selected_device) = main_window.selected_device() else {
                    tracing::warn!("no device is selected");
                    return;
                };
                let Some(device) = gtk_registry_clone.get_device_by_mac_address(&selected_device.mac_address).await else {
                    tracing::warn!("could not find selected device: {}", selected_device.mac_address);
                    return;
                };
                let configuration = device.get_equalizer_configuration().await;
                main_window.set_equalizer_configuration(configuration);
            }));
        }),
    );

    window.set_child(Some(&main_window));
    window.present();
}

fn load_resources() {
    gio::resources_register_include!("widgets.gresource")
        .expect("failed to load widgets");
}

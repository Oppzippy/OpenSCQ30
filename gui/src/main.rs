use std::sync::Arc;

use gtk::{
    gio,
    glib::{self, clone, closure_local, MainContext},
    prelude::{ApplicationExt, ApplicationExtManual, ObjectExt},
    traits::GtkWindowExt,
    ApplicationWindow,
};
use gtk_openscq30_lib::soundcore_device_registry::GtkSoundcoreDeviceRegistry;
use openscq30_lib::{
    api::soundcore_device_registry::SoundcoreDeviceRegistry,
    packets::structures::{
        ambient_sound_mode::AmbientSoundMode, noise_canceling_mode::NoiseCancelingMode,
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
        .with_max_level(Level::INFO)
        .init();

    load_resources();

    let app = adw::Application::builder()
        .application_id("com.oppzippy.openscq30")
        .build();
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &adw::Application) {
    let tokio_runtime = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(err) => panic!("failed to start tokio runtime: {err}"),
    };

    let registry = match tokio_runtime.block_on(SoundcoreDeviceRegistry::new()) {
        Ok(registry) => registry,
        Err(err) => panic!("failed to initialize device registry: {err}"),
    };

    tokio_runtime.block_on(registry.refresh_devices()).unwrap(); // TODO

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
                        None => todo!(),
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
                if let Some(selected_device) = main_window.selected_device() {
                    if let Some(device) = gtk_registry_clone.get_device_by_mac_address(&selected_device.mac_address).await {
                        let ambient_sound_mode = AmbientSoundMode::from_id(mode_id).unwrap();
                        device.set_ambient_sound_mode(ambient_sound_mode).await.unwrap();
                    } else {
                        tracing::warn!("could not find selected device: {}", selected_device.mac_address);
                    }
                }
            }));
        }),
    );

    let gtk_registry_clone = gtk_registry.clone();
    main_window.connect_closure(
        "noise-canceling-mode-selected",
        false,
        closure_local!(move |main_window: MainWindow, mode: u8| {
            let main_context = MainContext::default();
            main_context.spawn_local(clone!(@weak main_window, @weak gtk_registry_clone => async move {
                if let Some(selected_device) = main_window.selected_device() {
                    if let Some(device) = gtk_registry_clone.get_device_by_mac_address(&selected_device.mac_address).await {
                        let noise_canceling_mode = NoiseCancelingMode::from_id(mode).unwrap();
                        device.set_noise_canceling_mode(noise_canceling_mode).await.unwrap();
                    }else {
                        tracing::warn!("could not find selected device: {}", selected_device.mac_address);
                    }
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
                if let Some(selected_device) = main_window.selected_device() {
                    if let Some(device) = gtk_registry_clone.get_device_by_mac_address(&selected_device.mac_address).await {
                        let configuration = main_window.equalizer_configuration();
                        device.set_equalizer_configuration(configuration).await.unwrap();
                    } else {
                        tracing::warn!("could not find selected device: {}", selected_device.mac_address);
                    }
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
                if let Some(selected_device) = main_window.selected_device() {
                    if let Some(device) = gtk_registry_clone.get_device_by_mac_address(&selected_device.mac_address).await {
                        let configuration = device.get_equalizer_configuration().await;
                        main_window.set_equalizer_configuration(configuration);
                    } else {
                        tracing::warn!("could not find selected device: {}", selected_device.mac_address);
                    }
                }
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

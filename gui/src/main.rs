use std::sync::Arc;

use gtk::{
    gio,
    glib::{self, clone, closure_local, MainContext},
    prelude::{ApplicationExt, ApplicationExtManual, ObjectExt},
    traits::GtkWindowExt,
    ApplicationWindow,
};
use gtk_openscq30_lib::soundcore_device_registry::GtkSoundcoreDeviceRegistry;
use main_window::MainWindow;
use openscq30_lib::{
    api::soundcore_device_registry::SoundcoreDeviceRegistry,
    packets::structures::{
        ambient_sound_mode::AmbientSoundMode, noise_canceling_mode::NoiseCancelingMode,
    },
};

mod equalizer;
mod general_settings;
mod gtk_openscq30_lib;
mod main_window;
mod volume_slider;

fn main() {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
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

    let gtk_registry_1 = gtk_registry.clone();
    main_window.connect_closure(
        "ambient-sound-mode-selected",
        false,
        closure_local!(move |main_window: MainWindow, mode_id: u8| {
            let main_context = MainContext::default();
            let gtk_registry_2 = gtk_registry_1.to_owned();
            main_context.spawn_local(clone!(@weak main_window => async move {
                let ambient_sound_mode = AmbientSoundMode::from_id(mode_id).unwrap();
                let devices = gtk_registry_2.get_devices().await;
                let device = devices.first().unwrap();
                device.set_ambient_sound_mode(ambient_sound_mode).await.unwrap();
            }));
        }),
    );

    let gtk_registry_1 = gtk_registry.clone();
    main_window.connect_closure(
        "noise-canceling-mode-selected",
        false,
        closure_local!(move |main_window: MainWindow, mode: u8| {
            let main_context = MainContext::default();
            let gtk_registry_2 = gtk_registry_1.to_owned();
            main_context.spawn_local(clone!(@weak main_window => async move {
                let devices = gtk_registry_2.get_devices().await;
                devices.first().unwrap().set_noise_canceling_mode(NoiseCancelingMode::from_id(mode).unwrap()).await.unwrap();
            }));
        }),
    );

    window.set_child(Some(&main_window));
    window.present();
}

fn load_resources() {
    gio::resources_register_include!("volume_slider.gresource")
        .expect("failed to load volume slider");
    gio::resources_register_include!("equalizer.gresource").expect("failed to load volume slider");
    gio::resources_register_include!("general_settings.gresource").expect("failed to load general");
    gio::resources_register_include!("main_window.gresource").expect("failed to load main window");
}

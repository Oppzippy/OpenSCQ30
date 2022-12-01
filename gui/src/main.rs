use gtk::{
    gio,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::GtkWindowExt,
    ApplicationWindow,
};
use gtk_openscq30_lib::soundcore_device_registry::GtkSoundcoreDeviceRegistry;
use main_window::MainWindow;
use openscq30_lib::api::soundcore_device_registry::SoundcoreDeviceRegistry;
use tokio::runtime::Runtime;

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
    {
        let gtk_registry = GtkSoundcoreDeviceRegistry::new(registry, tokio_runtime);

        let app = adw::Application::builder()
            .application_id("com.oppzippy.openscq30")
            .build();
        app.connect_activate(move |app| build_ui(app, &gtk_registry));

        app.run();
    }
}

fn build_ui(app: &adw::Application, registry: &GtkSoundcoreDeviceRegistry) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("OpenSCQ30")
        .build();

    let main_window = MainWindow::new(registry);
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

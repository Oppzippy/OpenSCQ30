use gtk::{
    gio,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::GtkWindowExt,
    ApplicationWindow,
};
use main_window::MainWindow;

mod equalizer;
mod general;
mod main_window;
mod volume_slider;

fn main() {
    gio::resources_register_include!("volume_slider.gresource")
        .expect("failed to load volume slider");
    gio::resources_register_include!("equalizer.gresource").expect("failed to load volume slider");
    gio::resources_register_include!("general.gresource").expect("failed to load general");
    gio::resources_register_include!("main_window.gresource").expect("failed to load main window");

    let app = adw::Application::builder()
        .application_id("com.oppzippy.openscq30")
        .build();
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("OpenSCQ30")
        .build();

    let main_window = MainWindow::new();
    window.set_child(Some(&main_window));

    window.present();
}

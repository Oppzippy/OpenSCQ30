use equalizer::Equalizer;
use gtk::{
    gio,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::{BoxExt, GtkWindowExt},
    Application, ApplicationWindow,
};

mod equalizer;
mod volume_slider;

fn main() {
    gio::resources_register_include!("volume_slider.gresource")
        .expect("failed to load volume slider");
    gio::resources_register_include!("equalizer.gresource").expect("failed to load volume slider");

    let app = Application::builder()
        .application_id("com.oppzippy.openscq30")
        .build();
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("OpenSCQ30")
        .build();

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    window.set_child(Some(&container));

    let equalizer = Equalizer::new();
    container.append(&equalizer);

    window.present();
}

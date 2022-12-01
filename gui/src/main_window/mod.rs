mod imp;

use crate::gtk_openscq30_lib::soundcore_device_registry::GtkSoundcoreDeviceRegistry;
use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl MainWindow {
    pub fn new(registry: &GtkSoundcoreDeviceRegistry) -> Self {
        Object::new(&[])
    }
}

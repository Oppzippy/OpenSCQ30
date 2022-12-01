pub mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct GeneralSettings(ObjectSubclass<imp::GeneralSettings>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl GeneralSettings {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

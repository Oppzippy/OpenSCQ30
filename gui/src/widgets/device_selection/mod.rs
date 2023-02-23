mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

#[derive(Debug, Clone)]
pub struct Device {
    pub mac_address: String,
    pub name: String,
}

glib::wrapper! {
    pub struct DeviceSelection(ObjectSubclass<imp::DeviceSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DeviceSelection {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_devices(&self, devices: &[Device]) {
        self.imp().set_devices(devices)
    }
}

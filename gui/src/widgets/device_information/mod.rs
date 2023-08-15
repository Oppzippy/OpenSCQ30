mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::state::DeviceState;

glib::wrapper! {
    pub struct DeviceInformation(ObjectSubclass<imp::DeviceInformation>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DeviceInformation {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_device_state(&self, state: &DeviceState) {
        self.imp().set_device_state(state);
    }
}

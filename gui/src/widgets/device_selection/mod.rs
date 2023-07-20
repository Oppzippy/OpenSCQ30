mod imp;

use gtk::{
    glib::{self, Object, Sender},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::{actions::Action, objects::DeviceObject};

glib::wrapper! {
    pub struct DeviceSelection(ObjectSubclass<imp::DeviceSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DeviceSelection {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: Sender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_devices(&self, devices: &[DeviceObject]) {
        self.imp().set_devices(devices)
    }
}

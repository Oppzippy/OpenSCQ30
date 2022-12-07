use gtk::{
    glib::{self, Object},
    prelude::ObjectExt,
};

mod imp;

glib::wrapper! {
    pub struct DeviceObject(ObjectSubclass<imp::DeviceObject>);
}

impl DeviceObject {
    pub fn new(name: &String, mac_address: &String) -> Self {
        Object::new(&[("name", name), ("mac-address", mac_address)])
    }

    pub fn name(&self) -> String {
        self.property("name")
    }

    pub fn mac_address(&self) -> String {
        self.property("mac-address")
    }
}

#[derive(Default)]
pub struct DeviceData {}

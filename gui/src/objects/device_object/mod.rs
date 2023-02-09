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
        Object::builder()
            .property("name", name)
            .property("mac-address", mac_address)
            .build()
    }

    pub fn name(&self) -> String {
        self.property("name")
    }

    pub fn mac_address(&self) -> String {
        self.property("mac-address")
    }
}

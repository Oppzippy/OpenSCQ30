use gtk::glib::{self, Object};

mod imp;

glib::wrapper! {
    pub struct DeviceObject(ObjectSubclass<imp::DeviceObject>);
}

impl DeviceObject {
    pub fn new(name: &String, mac_address: &String) -> Self {
        Object::new(&[("name", name), ("mac-address", mac_address)])
    }
}

#[derive(Default)]
pub struct DeviceData {}

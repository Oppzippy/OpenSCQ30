use gtk::glib::{self, Object};

mod imp;

glib::wrapper! {
    pub struct DeviceObject(ObjectSubclass<imp::DeviceObject>);
}

impl DeviceObject {
    pub fn new(name: &str, mac_address: &str) -> Self {
        Object::builder()
            .property("name", name)
            .property("mac-address", mac_address)
            .build()
    }
}

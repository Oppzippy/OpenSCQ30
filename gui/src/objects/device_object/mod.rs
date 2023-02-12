use gtk::glib::{self, Object};

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
}

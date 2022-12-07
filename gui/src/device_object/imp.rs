use std::cell::RefCell;

use gtk::{
    glib::{self, once_cell::sync::Lazy, ParamSpec, ParamSpecString},
    prelude::ToValue,
    subclass::prelude::{ObjectImpl, ObjectSubclass},
};

#[derive(Default)]
pub struct DeviceObject {
    pub name: RefCell<String>,
    pub mac_address: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for DeviceObject {
    const NAME: &'static str = "OpenSCQ30DeviceObject";
    type Type = super::DeviceObject;
}

impl ObjectImpl for DeviceObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("name").build(),
                ParamSpecString::builder("mac-address").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let name = value.get().expect("name needs to be a string");
                self.name.replace(name);
            }
            "mac-address" => {
                let mac_address = value.get().expect("mac-address must be a string");
                self.mac_address.replace(mac_address);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            "mac-address" => self.mac_address.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}

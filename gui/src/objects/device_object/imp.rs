use std::cell::RefCell;

use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::ObjectExt,
    subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
};

#[derive(Default, Properties)]
#[properties(wrapper_type=super::DeviceObject)]
pub struct DeviceObject {
    #[property(set, get)]
    pub name: RefCell<String>,
    #[property(set, get)]
    pub mac_address: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for DeviceObject {
    const NAME: &'static str = "OpenSCQ30DeviceObject";
    type Type = super::DeviceObject;
}

impl ObjectImpl for DeviceObject {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        Self::derived_set_property(self, id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        Self::derived_property(self, id, pspec)
    }
}

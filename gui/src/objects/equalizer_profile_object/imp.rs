use std::cell::{Cell, RefCell};

use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::ObjectExt,
    subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
};

#[derive(Default, Properties)]
#[properties(wrapper_type = super::EqualizerProfileObject)]
pub struct EqualizerProfileObject {
    #[property(get, set, maximum = u16::MAX as u32)]
    pub profile_id: Cell<u32>,
    #[property(get, set)]
    pub name: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for EqualizerProfileObject {
    const NAME: &'static str = "OpenSCQ30EqualizerProfileObject";
    type Type = super::EqualizerProfileObject;
}

impl ObjectImpl for EqualizerProfileObject {
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

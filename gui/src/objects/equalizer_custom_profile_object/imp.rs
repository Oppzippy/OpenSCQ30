use std::cell::{Cell, RefCell};

use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::{ObjectExt, ParamSpecBuilderExt},
    subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
};

#[derive(Default, Properties)]
#[properties(wrapper_type = super::EqualizerCustomProfileObject)]
pub struct EqualizerCustomProfileObject {
    #[property(get, set)]
    pub name: RefCell<String>,
    pub volume_offsets: Cell<[i8; 8]>,
}

#[glib::object_subclass]
impl ObjectSubclass for EqualizerCustomProfileObject {
    const NAME: &'static str = "OpenSCQ30EqualizerCustomProfileObject";
    type Type = super::EqualizerCustomProfileObject;
}

impl ObjectImpl for EqualizerCustomProfileObject {
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

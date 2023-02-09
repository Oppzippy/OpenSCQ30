use std::cell::{Cell, RefCell};

use gtk::{
    glib::{self, once_cell::sync::Lazy, ParamSpec, ParamSpecString},
    prelude::ToValue,
    subclass::prelude::{ObjectImpl, ObjectSubclass},
};

#[derive(Default)]
pub struct EqualizerCustomProfileObject {
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
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecString::builder("name").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let name = value.get().expect("name needs to be a string");
                self.name.replace(name);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}

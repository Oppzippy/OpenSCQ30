use std::cell::{Cell, RefCell};

use gtk::{
    glib::{self, once_cell::sync::Lazy, ParamSpec, ParamSpecString, ParamSpecUInt},
    prelude::ToValue,
    subclass::prelude::{ObjectImpl, ObjectSubclass},
};

#[derive(Default)]
pub struct EqualizerProfileObject {
    pub profile_id: Cell<u32>,
    pub name: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for EqualizerProfileObject {
    const NAME: &'static str = "OpenSCQ30EqualizerProfileObject";
    type Type = super::EqualizerProfileObject;
}

impl ObjectImpl for EqualizerProfileObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("name").build(),
                ParamSpecUInt::builder("profile-id")
                    .maximum(u16::MAX as u32)
                    .build(),
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
            "profile-id" => {
                let profile_id: u32 = value.get().expect("profile-id must be a u32");
                self.profile_id.replace(profile_id);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            "profile-id" => self.profile_id.get().to_value(),
            _ => unimplemented!(),
        }
    }
}

use gtk::{
    glib::{self, Object},
    prelude::ObjectExt,
};

use crate::settings::EqualizerCustomProfile;

mod imp;

glib::wrapper! {
    pub struct EqualizerCustomProfileObject(ObjectSubclass<imp::EqualizerCustomProfileObject>);
}

impl EqualizerCustomProfileObject {
    pub fn new(name: &String) -> Self {
        Object::new(&[("name", name)])
    }

    pub fn name(&self) -> String {
        self.property("name")
    }
}

impl From<EqualizerCustomProfile> for EqualizerCustomProfileObject {
    fn from(custom_profile: EqualizerCustomProfile) -> Self {
        EqualizerCustomProfileObject::new(&custom_profile.name)
    }
}

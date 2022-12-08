use gtk::{
    glib::{self, Object},
    prelude::ObjectExt,
};

mod imp;

glib::wrapper! {
    pub struct EqualizerProfileObject(ObjectSubclass<imp::EqualizerProfileObject>);
}

impl EqualizerProfileObject {
    pub fn new(name: &String, profile_id: u32) -> Self {
        Object::new(&[("name", name), ("profile-id", &profile_id)])
    }

    pub fn name(&self) -> String {
        self.property("name")
    }

    pub fn profile_id(&self) -> u32 {
        self.property("profile_id")
    }
}

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
        Object::builder()
            .property("name", name)
            .property("profile-id", &profile_id)
            .build()
    }

    pub fn name(&self) -> String {
        self.property("name")
    }

    pub fn profile_id(&self) -> u32 {
        self.property("profile_id")
    }
}

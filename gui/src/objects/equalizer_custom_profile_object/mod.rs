use gtk::{
    glib::{self, Object},
    prelude::ObjectExt,
    subclass::prelude::ObjectSubclassIsExt,
};

mod imp;

glib::wrapper! {
    pub struct EqualizerCustomProfileObject(ObjectSubclass<imp::EqualizerCustomProfileObject>);
}

impl EqualizerCustomProfileObject {
    pub fn new(name: &String, volume_offsets: [i8; 8]) -> Self {
        let obj: Self = Object::new(&[("name", name)]);
        obj.imp().volume_offsets.replace(volume_offsets);
        obj
    }

    pub fn name(&self) -> String {
        self.property("name")
    }

    pub fn volume_offsets(&self) -> [i8; 8] {
        self.imp().volume_offsets.get()
    }
}

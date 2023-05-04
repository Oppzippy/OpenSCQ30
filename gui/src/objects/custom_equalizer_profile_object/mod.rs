use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

mod imp;

glib::wrapper! {
    pub struct CustomEqualizerProfileObject(ObjectSubclass<imp::CustomEqualizerProfileObject>);
}

impl CustomEqualizerProfileObject {
    pub fn new(name: &str, volume_offsets: [i8; 8]) -> Self {
        let obj: Self = Object::builder().property("name", name).build();
        obj.imp().volume_offsets.replace(volume_offsets);
        obj
    }

    pub fn volume_offsets(&self) -> [i8; 8] {
        self.imp().volume_offsets.get()
    }
}

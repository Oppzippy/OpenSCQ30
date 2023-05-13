use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

mod imp;

glib::wrapper! {
    pub struct CustomEqualizerProfileObject(ObjectSubclass<imp::CustomEqualizerProfileObject>);
}

impl CustomEqualizerProfileObject {
    pub fn new(name: &str, volume_adjustments: [i8; 8]) -> Self {
        let obj: Self = Object::builder().property("name", name).build();
        obj.imp().volume_adjustments.replace(volume_adjustments);
        obj
    }

    pub fn volume_adjustments(&self) -> [i8; 8] {
        self.imp().volume_adjustments.get()
    }
}

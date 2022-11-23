mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

glib::wrapper! {
    pub struct Equalizer(ObjectSubclass<imp::Equalizer>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Equalizer {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn volumes(&self) -> [i8; 8] {
        return self.imp().volumes();
    }
}

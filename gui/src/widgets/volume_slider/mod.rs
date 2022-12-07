mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

glib::wrapper! {
    pub struct VolumeSlider(ObjectSubclass<imp::VolumeSlider>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl VolumeSlider {
    pub fn new(band: i32, volume: f64) -> Self {
        Object::new(&[("band", &band), ("volume", &volume)])
    }

    pub fn volume(&self) -> i8 {
        self.imp().volume()
    }

    pub fn set_volume(&self, volume: i8) {
        self.imp().set_volume(volume)
    }
}

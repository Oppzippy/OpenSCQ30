mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct VolumeSlider(ObjectSubclass<imp::VolumeSlider>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl VolumeSlider {
    pub fn new(band: i32, volume: f64) -> Self {
        Object::builder()
            .property("band", &band)
            .property("volume", &volume)
            .build()
    }
}

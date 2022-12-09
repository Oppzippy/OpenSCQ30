pub mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::packets::structures::{AmbientSoundMode, NoiseCancelingMode};

glib::wrapper! {
    pub struct GeneralSettings(ObjectSubclass<imp::GeneralSettings>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl GeneralSettings {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn set_ambient_sound_mode(&self, ambient_sound_mode: AmbientSoundMode) {
        self.imp().set_ambient_sound_mode(ambient_sound_mode);
    }

    pub fn set_noise_canceling_mode(&self, noise_canceling_mode: NoiseCancelingMode) {
        self.imp().set_noise_canceling_mode(noise_canceling_mode);
    }
}

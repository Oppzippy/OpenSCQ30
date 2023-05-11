pub mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::packets::structures::EqualizerConfiguration;

use crate::objects::CustomEqualizerProfileObject;

glib::wrapper! {
    pub struct EqualizerSettings(ObjectSubclass<imp::EqualizerSettings>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EqualizerSettings {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_equalizer_configuration(&self, equalizer_configuration: &EqualizerConfiguration) {
        self.imp()
            .set_equalizer_configuration(equalizer_configuration);
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.imp().equalizer_configuration()
    }

    pub fn set_custom_profiles(&self, custom_profiles: Vec<CustomEqualizerProfileObject>) {
        self.imp().set_custom_profiles(custom_profiles)
    }
}

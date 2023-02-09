pub mod imp;

use gtk::{
    glib::{self, Object},
    prelude::ToValue,
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::packets::structures::EqualizerConfiguration;

use crate::objects::EqualizerCustomProfileObject;

glib::wrapper! {
    pub struct EqualizerSettings(ObjectSubclass<imp::EqualizerSettings>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EqualizerSettings {
    pub fn new() -> Self {
        Object::new(&[("is-custom-profile", &false.to_value())])
    }

    pub fn set_equalizer_configuration(&self, equalizer_configuration: EqualizerConfiguration) {
        self.imp()
            .set_equalizer_configuration(equalizer_configuration);
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.imp().equalizer_configuration()
    }

    pub fn set_custom_profiles(&self, custom_profiles: Vec<EqualizerCustomProfileObject>) {
        self.imp().set_custom_profiles(custom_profiles)
    }
}

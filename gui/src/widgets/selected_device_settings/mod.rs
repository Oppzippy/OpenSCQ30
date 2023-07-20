mod imp;

use gtk::{
    glib::{self, Object, Sender},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::packets::structures::{
    AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode,
};

use crate::{actions::Action, objects::CustomEqualizerProfileObject};

glib::wrapper! {
    pub struct SelectedDeviceSettings(ObjectSubclass<imp::SelectedDeviceSettings>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SelectedDeviceSettings {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: Sender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_ambient_sound_mode(&self, ambient_sound_mode: AmbientSoundMode) {
        self.imp()
            .general_settings
            .set_ambient_sound_mode(ambient_sound_mode);
    }

    pub fn set_noise_canceling_mode(&self, noise_canceling_mode: NoiseCancelingMode) {
        self.imp()
            .general_settings
            .set_noise_canceling_mode(noise_canceling_mode);
    }

    pub fn set_equalizer_configuration(&self, equalizer_configuration: &EqualizerConfiguration) {
        self.imp()
            .equalizer_settings
            .set_equalizer_configuration(equalizer_configuration);
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.imp().equalizer_settings.equalizer_configuration()
    }

    pub fn set_custom_profiles(&self, custom_profiles: Vec<CustomEqualizerProfileObject>) {
        self.imp()
            .equalizer_settings
            .set_custom_profiles(custom_profiles)
    }
}

mod imp;

use gtk::{
    glib::{self, Object, Sender},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::{packets::structures::EqualizerConfiguration, state::DeviceState};

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

    pub fn set_device_state(&self, state: &DeviceState) {
        self.imp().set_device_state(state);
    }

    pub fn set_equalizer_configuration(&self, equalizer_configuration: EqualizerConfiguration) {
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

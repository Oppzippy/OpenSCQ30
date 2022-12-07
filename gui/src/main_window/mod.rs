mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::packets::structures::{
    ambient_sound_mode::AmbientSoundMode, equalizer_band_offsets::EqualizerBandOffsets,
    equalizer_configuration::EqualizerConfiguration, noise_canceling_mode::NoiseCancelingMode,
};

use crate::device_selection::Device;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl MainWindow {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn set_devices(&self, devices: &[Device]) {
        self.imp().set_devices(devices);
    }

    pub fn selected_device(&self) -> Option<Device> {
        self.imp().device_selection.selected_device()
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

    pub fn set_equalizer_configuration(&self, equalizer_configuration: EqualizerConfiguration) {
        self.imp()
            .equalizer
            .set_volumes(equalizer_configuration.band_offsets().volume_offsets());
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        let volume_offsets = self.imp().equalizer.volumes();
        EqualizerConfiguration::Custom(EqualizerBandOffsets::new(volume_offsets))
    }
}

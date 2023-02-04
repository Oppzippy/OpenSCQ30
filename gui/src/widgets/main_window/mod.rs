mod imp;

use gtk::{
    glib::{self, Object},
    prelude::IsA,
    subclass::prelude::ObjectSubclassIsExt,
    Application,
};
use openscq30_lib::packets::structures::{
    AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode,
};

use super::Device;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(application: &impl IsA<Application>) -> Self {
        Object::builder()
            .property("application", application)
            .build()
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
            .equalizer_settings
            .set_equalizer_configuration(equalizer_configuration);
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.imp().equalizer_settings.equalizer_configuration()
    }
}

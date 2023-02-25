mod imp;

use std::rc::Rc;

use gtk::{
    gio,
    glib::{self, Object},
    prelude::IsA,
    subclass::prelude::ObjectSubclassIsExt,
    traits::GtkWindowExt,
    Application,
};
use openscq30_lib::packets::structures::{
    AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode,
};

use crate::{objects::EqualizerCustomProfileObject, settings::SettingsFile};

use super::Device;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionGroup, gio::ActionMap;
}

impl MainWindow {
    pub fn new(application: &impl IsA<Application>, settings_file: Rc<SettingsFile>) -> Self {
        let obj: Self = Object::builder()
            .property("application", application)
            .build();

        obj.imp()
            .settings_file
            .set(settings_file)
            .expect("must be able to set settings file");

        obj.load_window_size();

        obj
    }

    fn settings_file(&self) -> &SettingsFile {
        self.imp()
            .settings_file
            .get()
            .expect("settings must be set")
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        self.settings_file()
            .edit(|settings| {
                let size = self.default_size();
                settings.set_window_width(size.0);
                settings.set_window_height(size.1);
                settings.set_maximized(self.is_maximized());
            })
            .expect("failed to edit settings");

        Ok(())
    }

    fn load_window_size(&self) {
        self.settings_file()
            .get(|settings| {
                self.set_default_size(settings.window_width(), settings.window_height());
                if settings.is_maximized() {
                    self.maximize();
                }
            })
            .expect("failed to get settings");
    }

    pub fn set_devices(&self, devices: &[Device]) {
        self.imp().set_devices(devices);
    }

    pub fn set_ambient_sound_mode(&self, ambient_sound_mode: AmbientSoundMode) {
        self.imp()
            .selected_device_settings
            .imp()
            .general_settings
            .set_ambient_sound_mode(ambient_sound_mode);
    }

    pub fn set_noise_canceling_mode(&self, noise_canceling_mode: NoiseCancelingMode) {
        self.imp()
            .selected_device_settings
            .imp()
            .general_settings
            .set_noise_canceling_mode(noise_canceling_mode);
    }

    pub fn set_equalizer_configuration(&self, equalizer_configuration: &EqualizerConfiguration) {
        self.imp()
            .selected_device_settings
            .imp()
            .equalizer_settings
            .set_equalizer_configuration(&equalizer_configuration);
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.imp()
            .selected_device_settings
            .imp()
            .equalizer_settings
            .equalizer_configuration()
    }

    pub fn set_custom_profiles(&self, custom_profiles: Vec<EqualizerCustomProfileObject>) {
        self.imp()
            .selected_device_settings
            .imp()
            .equalizer_settings
            .set_custom_profiles(custom_profiles)
    }
}

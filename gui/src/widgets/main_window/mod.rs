mod imp;

use std::rc::Rc;

use adw::Toast;
use gtk::{
    gio,
    glib::{self, Object, Sender},
    prelude::IsA,
    subclass::prelude::ObjectSubclassIsExt,
    traits::GtkWindowExt,
    Application,
};
use openscq30_lib::{
    packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    state::DeviceState,
};

use crate::{
    actions::Action,
    objects::{CustomEqualizerProfileObject, DeviceObject},
    settings::Settings,
};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionGroup, gio::ActionMap;
}

impl MainWindow {
    pub fn new(application: &impl IsA<Application>, settings: Rc<Settings>) -> Self {
        let obj: Self = Object::builder()
            .property("application", application)
            .build();

        obj.imp()
            .settings
            .set(settings)
            .expect("must be able to set settings file");

        obj.load_window_size();

        obj
    }

    fn settings_file(&self) -> &Settings {
        self.imp().settings.get().expect("settings must be set")
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        self.settings_file()
            .state
            .edit(|settings| {
                let size = self.default_size();
                settings.window_width = size.0;
                settings.window_height = size.1;
                settings.is_maximized = self.is_maximized();
            })
            .expect("failed to edit settings");

        Ok(())
    }

    fn load_window_size(&self) {
        self.settings_file()
            .state
            .get(|settings| {
                self.set_default_size(settings.window_width, settings.window_height);
                if settings.is_maximized {
                    self.maximize();
                }
            })
            .expect("failed to get settings");
    }

    pub fn set_devices(&self, devices: &[DeviceObject]) {
        self.imp().set_devices(devices);
    }

    pub fn set_sender(&self, sender: Sender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_state(&self, state: &DeviceState) {
        self.set_equalizer_configuration(state.equalizer_configuration);
        if let Some(sound_modes) = state.sound_modes {
            self.set_ambient_sound_mode(sound_modes.ambient_sound_mode);
            self.set_noise_canceling_mode(sound_modes.noise_canceling_mode);
        }
    }

    pub fn set_ambient_sound_mode(&self, ambient_sound_mode: AmbientSoundMode) {
        self.imp()
            .selected_device_settings
            .set_ambient_sound_mode(ambient_sound_mode);
    }

    pub fn set_noise_canceling_mode(&self, noise_canceling_mode: NoiseCancelingMode) {
        self.imp()
            .selected_device_settings
            .set_noise_canceling_mode(noise_canceling_mode);
    }

    pub fn set_equalizer_configuration(&self, equalizer_configuration: EqualizerConfiguration) {
        self.imp()
            .selected_device_settings
            .set_equalizer_configuration(equalizer_configuration);
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.imp()
            .selected_device_settings
            .equalizer_configuration()
    }

    pub fn set_custom_profiles(&self, custom_profiles: Vec<CustomEqualizerProfileObject>) {
        self.imp()
            .selected_device_settings
            .set_custom_profiles(custom_profiles)
    }

    pub fn add_toast(&self, toast: Toast) {
        self.imp().toast_overlay.add_toast(toast);
    }
}

use gtk::{
    glib::{self, Object, Sender},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::{packets::structures::EqualizerConfiguration, state::DeviceState};

use crate::{
    actions::Action,
    objects::{GlibCustomEqualizerProfile, GlibNamedQuickPresetValue},
};

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

    pub fn set_custom_profiles(&self, custom_profiles: Vec<GlibCustomEqualizerProfile>) {
        self.imp()
            .equalizer_settings
            .set_custom_profiles(custom_profiles.to_owned());
        self.imp()
            .quick_presets
            .set_custom_profiles(custom_profiles);
    }

    pub fn set_quick_presets(&self, quick_presets: Vec<GlibNamedQuickPresetValue>) {
        self.imp().quick_presets.set_quick_presets(quick_presets)
    }
}

mod imp {
    use std::cell::OnceCell;

    use adw::prelude::WidgetExt;
    use gtk::{
        glib::{self, Sender},
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        CompositeTemplate, TemplateChild,
    };

    use openscq30_lib::{packets::structures::DeviceFeatureFlags, state::DeviceState};

    use crate::{
        actions::Action,
        ui::widgets::{
            DeviceInformation, EqualizerSettingsScreen, GeneralSettingsScreen, HearIdScreen,
            QuickPresetsScreen,
        },
    };

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/selected_device_settings.ui")]
    pub struct SelectedDeviceSettings {
        #[template_child]
        pub notebook: TemplateChild<gtk::Notebook>,
        #[template_child]
        pub general_settings: TemplateChild<GeneralSettingsScreen>,
        #[template_child]
        pub equalizer_settings: TemplateChild<EqualizerSettingsScreen>,
        #[template_child]
        pub hear_id: TemplateChild<HearIdScreen>,
        #[template_child]
        pub quick_presets: TemplateChild<QuickPresetsScreen>,
        #[template_child]
        pub device_information: TemplateChild<DeviceInformation>,

        sender: OnceCell<Sender<Action>>,
    }

    impl SelectedDeviceSettings {
        pub fn set_sender(&self, sender: Sender<Action>) {
            self.sender.set(sender.clone()).unwrap();
            self.general_settings.set_sender(sender.clone());
            self.equalizer_settings.set_sender(sender.clone());
            self.hear_id.set_sender(sender.clone());
            self.quick_presets.set_sender(sender.clone());
        }
    }

    impl SelectedDeviceSettings {
        pub fn set_device_state(&self, state: &DeviceState) {
            self.general_settings.set_device_state(state);
            self.equalizer_settings
                .set_equalizer_configuration(state.equalizer_configuration);
            self.device_information.set_device_state(state);
            self.quick_presets.set_device_state(state);

            if state.feature_flags.contains(DeviceFeatureFlags::HEAR_ID) && state.hear_id.is_some()
            {
                self.hear_id.set_visible(true);
                self.hear_id.set_device_state(state);
            } else {
                self.hear_id.set_visible(false);
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SelectedDeviceSettings {
        const NAME: &'static str = "OpenSCQ30SelectedDeviceSettings";
        type Type = super::SelectedDeviceSettings;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SelectedDeviceSettings {}
    impl WidgetImpl for SelectedDeviceSettings {}
    impl BoxImpl for SelectedDeviceSettings {}
}

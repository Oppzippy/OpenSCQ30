use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::devices::standard::state::DeviceState;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    actions::Action,
    objects::{GlibCustomEqualizerProfile, GlibNamedQuickPresetValue},
};

glib::wrapper! {
    pub struct QuickPresetsScreen(ObjectSubclass<imp::QuickPresetsScreen>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl QuickPresetsScreen {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: UnboundedSender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_quick_presets(&self, quick_presets: Vec<GlibNamedQuickPresetValue>) {
        self.imp()
            .quick_presets_listing
            .set_quick_presets(quick_presets)
    }

    pub fn set_device_state(&self, state: &DeviceState) {
        self.imp()
            .edit_quick_preset
            .set_device_profile(&state.device_profile);
    }

    pub fn set_custom_profiles(&self, custom_profiles: Vec<GlibCustomEqualizerProfile>) {
        self.imp()
            .edit_quick_preset
            .set_custom_equalizer_profiles(custom_profiles);
    }
}

mod imp {
    use gtk::{
        glib,
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, CompositeTemplate,
    };
    use once_cell::unsync::OnceCell;
    use tokio::sync::mpsc::UnboundedSender;

    use crate::{
        actions::Action,
        objects::GlibNamedQuickPresetValue,
        ui::widgets::quick_presets::{
            edit_quick_preset::EditQuickPreset, quick_presets_listing::QuickPresetsListing,
        },
    };

    #[derive(Default, CompositeTemplate)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/quick_presets/quick_presets_screen.ui"
    )]
    pub struct QuickPresetsScreen {
        #[template_child]
        pub navigation: TemplateChild<adw::NavigationView>,
        #[template_child]
        pub quick_presets_listing: TemplateChild<QuickPresetsListing>,
        #[template_child]
        pub edit_quick_preset: TemplateChild<EditQuickPreset>,

        sender: OnceCell<UnboundedSender<Action>>,
    }

    #[template_callbacks]
    impl QuickPresetsScreen {
        #[template_callback]
        pub fn handle_create_quick_preset(
            &self,
            named_quick_preset: GlibNamedQuickPresetValue,
            _: &QuickPresetsListing,
        ) {
            self.sender
                .get()
                .unwrap()
                .send(Action::CreateQuickPreset(named_quick_preset))
                .unwrap();
        }

        #[template_callback]
        pub fn handle_edit_quick_preset(
            &self,
            named_quick_preset: GlibNamedQuickPresetValue,
            _: &QuickPresetsListing,
        ) {
            self.edit_quick_preset.set_quick_preset(named_quick_preset);
            self.navigation.push_by_tag("edit-quick-preset");
        }

        #[template_callback]
        fn handle_activate_quick_preset(
            &self,
            quick_preset: GlibNamedQuickPresetValue,
            _: &QuickPresetsListing,
        ) {
            self.sender
                .get()
                .unwrap()
                .send(Action::ActivateQuickPreset(quick_preset))
                .unwrap();
        }

        #[template_callback]
        fn handle_delete_quick_preset(
            &self,
            quick_preset: GlibNamedQuickPresetValue,
            _: &QuickPresetsListing,
        ) {
            self.sender
                .get()
                .unwrap()
                .send(Action::DeleteQuickPreset(quick_preset.name))
                .unwrap();
        }

        #[template_callback]
        fn handle_quick_preset_changed(
            &self,
            quick_preset: GlibNamedQuickPresetValue,
            _: &EditQuickPreset,
        ) {
            self.sender
                .get()
                .unwrap()
                .send(Action::CreateQuickPreset(quick_preset))
                .unwrap();
        }
    }

    impl QuickPresetsScreen {
        pub fn set_sender(&self, sender: UnboundedSender<Action>) {
            self.sender.set(sender.to_owned()).unwrap();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for QuickPresetsScreen {
        const NAME: &'static str = "OpenSCQ30QuickPresetsScreen";
        type Type = super::QuickPresetsScreen;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for QuickPresetsScreen {
        fn constructed(&self) {}
    }
    impl WidgetImpl for QuickPresetsScreen {}
    impl BoxImpl for QuickPresetsScreen {}
}

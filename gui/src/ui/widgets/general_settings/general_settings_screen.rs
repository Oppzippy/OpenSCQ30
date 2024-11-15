use crate::actions::Action;
use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::devices::standard::state::DeviceState;
use tokio::sync::mpsc::UnboundedSender;

glib::wrapper! {
    pub struct GeneralSettingsScreen(ObjectSubclass<imp::GeneralSettingsScreen>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl GeneralSettingsScreen {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: UnboundedSender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_device_state(&self, state: &DeviceState) {
        self.imp().set_device_state(state);
    }
}

mod imp {
    use crate::{
        actions::Action,
        ui::widgets::general_settings::{
            sound_modes::SoundModes, sound_modes_type_two::SoundModesTypeTwo,
        },
    };
    use gtk::{
        glib,
        prelude::WidgetExt,
        subclass::{
            prelude::*,
            widget::{
                CompositeTemplateCallbacksClass, CompositeTemplateClass,
                CompositeTemplateInitializingExt, WidgetImpl,
            },
        },
        CompositeTemplate, TemplateChild,
    };
    use openscq30_lib::devices::standard::state::DeviceState;
    use std::cell::OnceCell;
    use tokio::sync::mpsc::UnboundedSender;

    #[derive(Default, CompositeTemplate)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/general_settings_screen.ui"
    )]
    pub struct GeneralSettingsScreen {
        #[template_child]
        pub sound_modes: TemplateChild<SoundModes>,
        #[template_child]
        pub sound_modes_type_two: TemplateChild<SoundModesTypeTwo>,

        sender: OnceCell<UnboundedSender<Action>>,
    }

    #[gtk::template_callbacks]
    impl GeneralSettingsScreen {
        pub fn set_sender(&self, sender: UnboundedSender<Action>) {
            self.sound_modes.set_sender(sender.clone());
            self.sound_modes_type_two.set_sender(sender.clone());
            self.sender.set(sender).unwrap();
        }

        pub fn set_device_state(&self, state: &DeviceState) {
            self.sound_modes.set_device_state(state);
            self.sound_modes.set_visible(state.sound_modes.is_some());
            self.sound_modes_type_two.set_device_state(state);
            self.sound_modes_type_two
                .set_visible(state.sound_modes_type_two.is_some());
        }

        fn send_action(&self, action: Action) {
            self.sender.get().unwrap().send(action).unwrap();
        }

        #[template_callback]
        fn handle_disconnect_clicked(&self, _: &gtk::Button) {
            self.send_action(Action::Disconnect);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GeneralSettingsScreen {
        const NAME: &'static str = "OpenSCQ30GeneralSettingsScreen";
        type Type = super::GeneralSettingsScreen;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GeneralSettingsScreen {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for GeneralSettingsScreen {}
    impl BoxImpl for GeneralSettingsScreen {}
}

use gtk::{
    glib::{self, Object, Sender},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::state::DeviceState;

use crate::actions::Action;

glib::wrapper! {
    pub struct GeneralSettingsScreen(ObjectSubclass<imp::GeneralSettingsScreen>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl GeneralSettingsScreen {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: Sender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_device_state(&self, state: &DeviceState) {
        self.imp().set_device_state(state);
    }
}

mod imp {
    // Properties macro creates an enum for internal use. We don't care that it is caught by this lint.
    #![allow(clippy::enum_variant_names)]

    use std::cell::OnceCell;

    use gtk::{
        glib::{self, Sender},
        prelude::*,
        subclass::{
            prelude::*,
            widget::{
                CompositeTemplateCallbacksClass, CompositeTemplateClass,
                CompositeTemplateInitializingExt, WidgetImpl,
            },
        },
        CompositeTemplate, TemplateChild,
    };
    use openscq30_lib::{packets::structures::DeviceFeatureFlags, state::DeviceState};

    use crate::{
        actions::Action,
        objects::{
            BoxedAmbientSoundMode, BoxedCustomNoiseCanceling, BoxedNoiseCancelingMode,
            BoxedTransparencyMode,
        },
        ui::widgets::general_settings::{
            ambient_sound_mode_selection::AmbientSoundModeSelection,
            custom_noise_canceling_selection::CustomNoiseCancelingSelection,
            noise_canceling_mode_selection::NoiseCancelingModeSelection,
            transparency_mode_selection::TransparencyModeSelection,
        },
    };

    #[derive(Default, CompositeTemplate)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/general_settings_screen.ui"
    )]
    pub struct GeneralSettingsScreen {
        // Ambient Sound Mode
        #[template_child]
        pub ambient_sound_mode_selection: TemplateChild<AmbientSoundModeSelection>,

        // Transpareny Mode
        #[template_child]
        pub transparency_mode_selection: TemplateChild<TransparencyModeSelection>,

        // Noise Canceling Mode
        #[template_child]
        pub noise_canceling_mode_selection: TemplateChild<NoiseCancelingModeSelection>,

        // Custom noise canceling
        #[template_child]
        pub custom_noise_canceling_selection: TemplateChild<CustomNoiseCancelingSelection>,

        sender: OnceCell<Sender<Action>>,
    }

    #[gtk::template_callbacks]
    impl GeneralSettingsScreen {
        pub fn set_sender(&self, sender: Sender<Action>) {
            self.sender.set(sender.clone()).unwrap();
        }

        pub fn set_device_state(&self, state: &DeviceState) {
            let Some(sound_modes) = state.sound_modes else {
                return;
            };

            // Set button visibility
            self.ambient_sound_mode_selection
                .set_has_noise_canceling_mode(
                    state
                        .feature_flags
                        .contains(DeviceFeatureFlags::NOISE_CANCELING_MODE),
                );
            self.transparency_mode_selection.set_visible(
                state
                    .feature_flags
                    .contains(DeviceFeatureFlags::TRANSPARENCY_MODES),
            );
            self.noise_canceling_mode_selection.set_visible(
                state
                    .feature_flags
                    .contains(DeviceFeatureFlags::NOISE_CANCELING_MODE),
            );
            self.noise_canceling_mode_selection
                .set_has_custom_noise_canceling(
                    state
                        .feature_flags
                        .contains(DeviceFeatureFlags::CUSTOM_NOISE_CANCELING),
                );
            self.custom_noise_canceling_selection.set_visible(
                state
                    .feature_flags
                    .contains(DeviceFeatureFlags::CUSTOM_NOISE_CANCELING),
            );

            // Set selected values
            self.ambient_sound_mode_selection
                .set_ambient_sound_mode(BoxedAmbientSoundMode(sound_modes.ambient_sound_mode));
            self.transparency_mode_selection
                .set_transparency_mode(BoxedTransparencyMode(sound_modes.transparency_mode));
            self.noise_canceling_mode_selection
                .set_noise_canceling_mode(BoxedNoiseCancelingMode(
                    sound_modes.noise_canceling_mode,
                ));
            self.custom_noise_canceling_selection
                .set_custom_noise_canceling(BoxedCustomNoiseCanceling(
                    sound_modes.custom_noise_canceling,
                ));
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

            // Ambient sound mode
            {
                let this = self.to_owned();
                self.ambient_sound_mode_selection
                    .connect_ambient_sound_mode_notify(move |ambient_sound_mode_selection| {
                        this.send_action(Action::SetAmbientSoundMode(
                            ambient_sound_mode_selection.ambient_sound_mode().0,
                        ));
                    });
            }
            // Transparency mode
            {
                let this = self.to_owned();
                self.transparency_mode_selection
                    .connect_transparency_mode_notify(move |transparency_mode_selection| {
                        this.send_action(Action::SetTransparencyMode(
                            transparency_mode_selection.transparency_mode().0,
                        ));
                    });
            }
            // Noise canceling mode
            {
                let this = self.to_owned();
                self.noise_canceling_mode_selection
                    .connect_noise_canceling_mode_notify(move |noise_canceling_mode_selection| {
                        this.send_action(Action::SetNoiseCancelingMode(
                            noise_canceling_mode_selection.noise_canceling_mode().0,
                        ));
                    });
            }
            // Custom noise canceling
            {
                let this = self.to_owned();
                self.custom_noise_canceling_selection
                    .connect_custom_noise_canceling_notify(
                        move |custom_noise_canceling_selection| {
                            this.send_action(Action::SetCustomNoiseCanceling(
                                custom_noise_canceling_selection.custom_noise_canceling().0,
                            ));
                        },
                    );
            }
        }
    }
    impl WidgetImpl for GeneralSettingsScreen {}
    impl BoxImpl for GeneralSettingsScreen {}
}

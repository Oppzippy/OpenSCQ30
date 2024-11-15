use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::devices::standard::state::DeviceState;
use tokio::sync::mpsc::UnboundedSender;

use crate::actions::Action;

glib::wrapper! {
    pub struct SoundModes(ObjectSubclass<imp::SoundModes>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl SoundModes {
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
    use std::cell::OnceCell;

    use gtk::{
        glib,
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
    use openscq30_lib::{
        device_profile::{NoiseCancelingModeType, TransparencyModeType},
        devices::standard::state::DeviceState,
    };
    use tokio::sync::mpsc::UnboundedSender;

    use crate::{
        actions::Action,
        objects::{
            GlibAmbientSoundModeCycleValue, GlibAmbientSoundModeValue,
            GlibCustomNoiseCancelingValue, GlibNoiseCancelingModeValue, GlibTransparencyModeValue,
        },
        ui::widgets::general_settings::{
            ambient_sound_mode_cycle_selection::AmbientSoundModeCycleSelection,
            ambient_sound_mode_selection::AmbientSoundModeSelection,
            custom_noise_canceling_selection::CustomNoiseCancelingSelection,
            noise_canceling_mode_selection::NoiseCancelingModeSelection,
            transparency_mode_selection::TransparencyModeSelection,
        },
    };

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/sound_modes.ui")]
    pub struct SoundModes {
        // Ambient Sound Mode
        #[template_child]
        pub ambient_sound_mode_selection: TemplateChild<AmbientSoundModeSelection>,

        // Ambient Sound Mode
        #[template_child]
        pub ambient_sound_mode_cycle_selection: TemplateChild<AmbientSoundModeCycleSelection>,

        // Transpareny Mode
        #[template_child]
        pub transparency_mode_selection: TemplateChild<TransparencyModeSelection>,

        // Noise Canceling Mode
        #[template_child]
        pub noise_canceling_mode_selection: TemplateChild<NoiseCancelingModeSelection>,

        // Custom noise canceling
        #[template_child]
        pub custom_noise_canceling_selection: TemplateChild<CustomNoiseCancelingSelection>,

        sender: OnceCell<UnboundedSender<Action>>,
    }

    #[gtk::template_callbacks]
    impl SoundModes {
        pub fn set_sender(&self, sender: UnboundedSender<Action>) {
            self.sender.set(sender).unwrap();
        }

        pub fn set_device_state(&self, state: &DeviceState) {
            let Some(sound_modes) = state.sound_modes else {
                return;
            };
            let Some(sound_mode_profile) = state.device_features.sound_mode else {
                return;
            };

            // Set button visibility
            self.ambient_sound_mode_selection
                .set_has_noise_canceling_mode(
                    sound_mode_profile.noise_canceling_mode_type != NoiseCancelingModeType::None,
                );
            self.ambient_sound_mode_cycle_selection
                .set_visible(state.device_features.has_ambient_sound_mode_cycle);
            self.ambient_sound_mode_cycle_selection
                .set_has_noise_canceling_mode(
                    sound_mode_profile.noise_canceling_mode_type != NoiseCancelingModeType::None,
                );
            self.transparency_mode_selection.set_visible(
                sound_mode_profile.transparency_mode_type == TransparencyModeType::Custom,
            );
            self.noise_canceling_mode_selection.set_visible(
                sound_mode_profile.noise_canceling_mode_type != NoiseCancelingModeType::None,
            );
            self.noise_canceling_mode_selection
                .set_has_custom_noise_canceling(
                    sound_mode_profile.noise_canceling_mode_type == NoiseCancelingModeType::Custom,
                );
            self.custom_noise_canceling_selection.set_visible(
                sound_mode_profile.noise_canceling_mode_type == NoiseCancelingModeType::Custom,
            );

            // Set selected values
            self.ambient_sound_mode_selection
                .set_ambient_sound_mode(GlibAmbientSoundModeValue(sound_modes.ambient_sound_mode));
            if let Some(cycle) = state.ambient_sound_mode_cycle {
                self.ambient_sound_mode_cycle_selection
                    .set_ambient_sound_mode_cycle(&cycle);
            }
            self.transparency_mode_selection
                .set_transparency_mode(GlibTransparencyModeValue(sound_modes.transparency_mode));
            self.noise_canceling_mode_selection
                .set_noise_canceling_mode(GlibNoiseCancelingModeValue(
                    sound_modes.noise_canceling_mode,
                ));
            self.custom_noise_canceling_selection
                .set_custom_noise_canceling(GlibCustomNoiseCancelingValue(
                    sound_modes.custom_noise_canceling,
                ));
        }

        fn send_action(&self, action: Action) {
            self.sender.get().unwrap().send(action).unwrap();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SoundModes {
        const NAME: &'static str = "OpenSCQ30SoundModes";
        type Type = super::SoundModes;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SoundModes {
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
            // Ambient sound mode cycle
            {
                let this = self.to_owned();
                self.ambient_sound_mode_cycle_selection.connect_local(
                    "ambient-sound-mode-cycle-changed",
                    false,
                    move |parameters| {
                        let cycle: GlibAmbientSoundModeCycleValue = parameters[1].get().unwrap();
                        this.send_action(Action::SetAmbientSoundModeCycle(cycle.0));
                        None
                    },
                );
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
    impl WidgetImpl for SoundModes {}
    impl BoxImpl for SoundModes {}
}

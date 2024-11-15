use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::devices::standard::state::DeviceState;
use tokio::sync::mpsc::UnboundedSender;

use crate::actions::Action;

glib::wrapper! {
    pub struct SoundModesTypeTwo(ObjectSubclass<imp::SoundModesTypeTwo>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl SoundModesTypeTwo {
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
    use openscq30_lib::devices::standard::{
        state::DeviceState, structures::NoiseCancelingModeTypeTwo,
    };
    use tokio::sync::mpsc::UnboundedSender;

    use crate::{
        actions::Action,
        objects::{
            GlibAdaptiveNoiseCancelingValue, GlibAmbientSoundModeCycleValue,
            GlibAmbientSoundModeValue, GlibManualNoiseCancelingValue,
            GlibNoiseCancelingModeTypeTwoValue, GlibTransparencyModeValue,
        },
        ui::widgets::general_settings::{
            adaptive_noise_canceling_selection::AdaptiveNoiseCancelingSelection,
            ambient_sound_mode_cycle_selection::AmbientSoundModeCycleSelection,
            ambient_sound_mode_selection::AmbientSoundModeSelection,
            manual_noise_canceling_selection::ManualNoiseCancelingSelection,
            noise_canceling_mode_type_two_selection::NoiseCancelingModeTypeTwoSelection,
            transparency_mode_selection::TransparencyModeSelection,
        },
    };

    #[derive(Default, CompositeTemplate)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/sound_modes_type_two.ui"
    )]
    pub struct SoundModesTypeTwo {
        #[template_child]
        pub ambient_sound_mode_selection: TemplateChild<AmbientSoundModeSelection>,
        #[template_child]
        pub ambient_sound_mode_cycle_selection: TemplateChild<AmbientSoundModeCycleSelection>,
        #[template_child]
        pub transparency_mode_selection: TemplateChild<TransparencyModeSelection>,
        #[template_child]
        pub noise_canceling_mode_selection: TemplateChild<NoiseCancelingModeTypeTwoSelection>,
        #[template_child]
        pub adaptive_noise_canceling_selection: TemplateChild<AdaptiveNoiseCancelingSelection>,
        #[template_child]
        pub manual_noise_canceling_selection: TemplateChild<ManualNoiseCancelingSelection>,

        sender: OnceCell<UnboundedSender<Action>>,
    }

    #[gtk::template_callbacks]
    impl SoundModesTypeTwo {
        pub fn set_sender(&self, sender: UnboundedSender<Action>) {
            self.sender.set(sender).unwrap();
        }

        pub fn set_device_state(&self, state: &DeviceState) {
            let Some(sound_modes) = state.sound_modes_type_two else {
                return;
            };

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
                .set_noise_canceling_mode(GlibNoiseCancelingModeTypeTwoValue(
                    sound_modes.noise_canceling_mode,
                ));
            self.adaptive_noise_canceling_selection
                .set_adaptive_noise_canceling(GlibAdaptiveNoiseCancelingValue(
                    sound_modes.adaptive_noise_canceling,
                ));
            self.manual_noise_canceling_selection
                .set_manual_noise_canceling(GlibManualNoiseCancelingValue(
                    sound_modes.manual_noise_canceling,
                ));
        }

        fn send_action(&self, action: Action) {
            self.sender.get().unwrap().send(action).unwrap();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SoundModesTypeTwo {
        const NAME: &'static str = "OpenSCQ30SoundModesTypeTwo";
        type Type = super::SoundModesTypeTwo;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SoundModesTypeTwo {
        fn constructed(&self) {
            self.parent_constructed();

            self.noise_canceling_mode_selection
                .bind_property(
                    "noise-canceling-mode",
                    &self.adaptive_noise_canceling_selection.get(),
                    "visible",
                )
                .transform_to(
                    |_, noise_canceling_mode: GlibNoiseCancelingModeTypeTwoValue| {
                        Some(noise_canceling_mode.0 == NoiseCancelingModeTypeTwo::Adaptive)
                    },
                )
                .sync_create()
                .build();
            self.noise_canceling_mode_selection
                .bind_property(
                    "noise-canceling-mode",
                    &self.manual_noise_canceling_selection.get(),
                    "visible",
                )
                .transform_to(
                    |_, noise_canceling_mode: GlibNoiseCancelingModeTypeTwoValue| {
                        Some(noise_canceling_mode.0 == NoiseCancelingModeTypeTwo::Manual)
                    },
                )
                .sync_create()
                .build();

            // Ambient sound mode
            {
                let this = self.to_owned();
                self.ambient_sound_mode_selection
                    .connect_ambient_sound_mode_notify(move |ambient_sound_mode_selection| {
                        this.send_action(Action::SetAmbientSoundModeTypeTwo(
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
                        this.send_action(Action::SetTransparencyModeTypeTwo(
                            transparency_mode_selection.transparency_mode().0,
                        ));
                    });
            }
            // Noise canceling mode
            {
                let this = self.to_owned();
                self.noise_canceling_mode_selection
                    .connect_noise_canceling_mode_notify(move |noise_canceling_mode_selection| {
                        this.send_action(Action::SetNoiseCancelingModeTypeTwo(
                            noise_canceling_mode_selection.noise_canceling_mode().0,
                        ));
                    });
            }
            // Manual noise canceling
            {
                let this = self.to_owned();
                self.manual_noise_canceling_selection
                    .connect_manual_noise_canceling_notify(
                        move |manual_noise_canceling_selection| {
                            this.send_action(Action::SetManualNoiseCanceling(
                                manual_noise_canceling_selection.manual_noise_canceling().0,
                            ));
                        },
                    );
            }
        }
    }
    impl WidgetImpl for SoundModesTypeTwo {}
    impl BoxImpl for SoundModesTypeTwo {}
}

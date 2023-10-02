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

    use std::cell::{Cell, OnceCell};

    use gtk::{
        glib::{self, ParamSpec, Properties, Sender, Value},
        prelude::*,
        subclass::{
            prelude::*,
            widget::{
                CompositeTemplateCallbacksClass, CompositeTemplateClass,
                CompositeTemplateInitializingExt, WidgetClassSubclassExt, WidgetImpl,
            },
        },
        traits::ToggleButtonExt,
        CompositeTemplate, TemplateChild,
    };
    use openscq30_lib::{
        packets::structures::{
            AmbientSoundMode, CustomNoiseCanceling, DeviceFeatureFlags, NoiseCancelingMode,
            TransparencyMode,
        },
        state::DeviceState,
    };

    use crate::{
        actions::Action, objects::BoxedAmbientSoundMode,
        ui::widgets::general_settings::ambient_sound_mode_selection::AmbientSoundModeSelection,
    };

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/general_settings_screen.ui"
    )]
    #[properties(wrapper_type=super::GeneralSettingsScreen)]
    pub struct GeneralSettingsScreen {
        // Ambient Sound Mode
        #[template_child]
        pub ambient_sound_mode_selection: TemplateChild<AmbientSoundModeSelection>,

        // Transpareny Mode
        #[template_child]
        pub transparency_mode_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub fully_transparent: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub vocal_mode: TemplateChild<gtk::ToggleButton>,

        // Noise Canceling Mode
        #[template_child]
        pub noise_canceling_mode_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub transport_mode: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub indoor_mode: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub outdoor_mode: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub custom_mode: TemplateChild<gtk::ToggleButton>,

        #[template_child]
        pub custom_noise_canceling_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub custom_noise_canceling: TemplateChild<gtk::Scale>,

        // The buttons fire their click signals when using set_active to set them in their initial states
        // We don't want to fire events to set the headphones to the state that they're already in,
        // so we can set this flag to true when we don't want to fire events up the chain.
        ignore_button_clicks: Cell<bool>,

        #[property(set, get)]
        has_noise_canceling_mode: Cell<bool>,
        #[property(set, get)]
        has_custom_noise_canceling: Cell<bool>,
        #[property(set, get)]
        has_custom_transparency_modes: Cell<bool>,

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
            let obj = self.obj();
            self.ambient_sound_mode_selection
                .set_device_feature_flags(&state.feature_flags);
            obj.set_has_noise_canceling_mode(
                state
                    .feature_flags
                    .contains(DeviceFeatureFlags::NOISE_CANCELING_MODE),
            );
            obj.set_has_custom_noise_canceling(
                state
                    .feature_flags
                    .contains(DeviceFeatureFlags::CUSTOM_NOISE_CANCELING),
            );
            obj.set_has_custom_transparency_modes(
                state
                    .feature_flags
                    .contains(DeviceFeatureFlags::TRANSPARENCY_MODES),
            );

            self.ambient_sound_mode_selection
                .set_ambient_sound_mode(BoxedAmbientSoundMode(sound_modes.ambient_sound_mode));
            self.set_transparency_mode(sound_modes.transparency_mode);
            self.set_noise_canceling_mode(sound_modes.noise_canceling_mode);
            self.set_custom_noise_canceling(sound_modes.custom_noise_canceling);
        }

        fn set_noise_canceling_mode(&self, noise_canceling_mode: NoiseCancelingMode) {
            self.ignore_button_clicks.replace(true);
            let button = match noise_canceling_mode {
                NoiseCancelingMode::Indoor => &self.indoor_mode,
                NoiseCancelingMode::Outdoor => &self.outdoor_mode,
                NoiseCancelingMode::Transport => &self.transport_mode,
                NoiseCancelingMode::Custom => &self.custom_mode,
            };
            button.set_active(true);
            self.ignore_button_clicks.replace(false);
        }

        fn set_transparency_mode(&self, transparency_mode: TransparencyMode) {
            self.ignore_button_clicks.replace(true);
            let button = match transparency_mode {
                TransparencyMode::FullyTransparent => &self.fully_transparent,
                TransparencyMode::VocalMode => &self.vocal_mode,
            };
            button.set_active(true);
            self.ignore_button_clicks.replace(false);
        }

        fn set_custom_noise_canceling(&self, custom_noise_canceling: CustomNoiseCanceling) {
            self.ignore_button_clicks.replace(true);
            self.custom_noise_canceling
                .set_value(custom_noise_canceling.value().into());
            self.ignore_button_clicks.replace(false);
        }

        fn send_action(&self, action: Action) {
            self.sender.get().unwrap().send(action).unwrap();
        }

        // Ambient Sound Mode
        #[template_callback]
        fn handle_ambient_sound_mode_change(&self, button: &gtk::ToggleButton) {
            if button.is_active() && !self.ignore_button_clicks.get() {
                self.send_action(Action::SetAmbientSoundMode(AmbientSoundMode::Normal));
            }
        }

        // Transparency Mode

        #[template_callback]
        fn handle_vocal_mode_clicked(&self, button: &gtk::ToggleButton) {
            if button.is_active() && !self.ignore_button_clicks.get() {
                self.send_action(Action::SetTransparencyMode(TransparencyMode::VocalMode));
            }
        }

        #[template_callback]
        fn handle_fully_transparent_clicked(&self, button: &gtk::ToggleButton) {
            if button.is_active() && !self.ignore_button_clicks.get() {
                self.send_action(Action::SetTransparencyMode(
                    TransparencyMode::FullyTransparent,
                ));
            }
        }

        // Noise Canceling Mode

        #[template_callback]
        fn handle_transport_mode_clicked(&self, button: &gtk::ToggleButton) {
            if button.is_active() && !self.ignore_button_clicks.get() {
                self.send_action(Action::SetNoiseCancelingMode(NoiseCancelingMode::Transport));
            }
        }

        #[template_callback]
        fn handle_indoor_mode_clicked(&self, button: &gtk::ToggleButton) {
            if button.is_active() && !self.ignore_button_clicks.get() {
                self.send_action(Action::SetNoiseCancelingMode(NoiseCancelingMode::Indoor));
            }
        }

        #[template_callback]
        fn handle_outdoor_mode_clicked(&self, button: &gtk::ToggleButton) {
            if button.is_active() && !self.ignore_button_clicks.get() {
                self.send_action(Action::SetNoiseCancelingMode(NoiseCancelingMode::Outdoor));
            }
        }

        #[template_callback]
        fn handle_custom_mode_clicked(&self, button: &gtk::ToggleButton) {
            if button.is_active() && !self.ignore_button_clicks.get() {
                self.send_action(Action::SetNoiseCancelingMode(NoiseCancelingMode::Custom));
            }
        }

        // Custom noise canceling

        #[template_callback]
        fn handle_custom_noise_canceling_changed(&self, scale: &gtk::Scale) {
            if !self.ignore_button_clicks.get() {
                self.send_action(Action::SetCustomNoiseCanceling(CustomNoiseCanceling::new(
                    scale.value() as u8,
                )));
            }
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
            let obj = self.obj();

            {
                let this = self.to_owned();
                self.ambient_sound_mode_selection
                    .connect_ambient_sound_mode_notify(move |ambient_sound_mode_selection| {
                        this.send_action(Action::SetAmbientSoundMode(
                            ambient_sound_mode_selection.ambient_sound_mode().0,
                        ));
                    });
            }

            obj.bind_property(
                "has_noise_canceling_mode",
                &self.noise_canceling_mode_label.get(),
                "visible",
            )
            .sync_create()
            .build();
            [
                &self.indoor_mode.get(),
                &self.outdoor_mode.get(),
                &self.transport_mode.get(),
            ]
            .into_iter()
            .for_each(|button| {
                obj.bind_property("has_noise_canceling_mode", button, "visible")
                    .sync_create()
                    .build();
            });

            obj.bind_property(
                "has_custom_noise_canceling",
                &self.custom_mode.get(),
                "visible",
            )
            .sync_create()
            .build();
            obj.bind_property(
                "has_custom_noise_canceling",
                &self.custom_noise_canceling_label.get(),
                "visible",
            )
            .sync_create()
            .build();
            obj.bind_property(
                "has_custom_noise_canceling",
                &self.custom_noise_canceling.get(),
                "visible",
            )
            .sync_create()
            .build();

            obj.bind_property(
                "has_custom_transparency_modes",
                &self.transparency_mode_label.get(),
                "visible",
            )
            .sync_create()
            .build();
            [&self.fully_transparent.get(), &self.vocal_mode.get()]
                .into_iter()
                .for_each(|button| {
                    obj.bind_property("has_custom_transparency_modes", button, "visible")
                        .sync_create()
                        .build();
                });
        }
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }
        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }
        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }
    impl WidgetImpl for GeneralSettingsScreen {}
    impl BoxImpl for GeneralSettingsScreen {}
}

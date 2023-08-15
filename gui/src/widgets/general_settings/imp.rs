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

use crate::actions::Action;

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/com/oppzippy/OpenSCQ30/general_settings/template.ui")]
#[properties(wrapper_type=super::GeneralSettings)]
pub struct GeneralSettings {
    // Ambient Sound Mode
    #[template_child]
    pub ambient_sound_mode_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub normal_mode: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub transparency_mode: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub noise_canceling_mode: TemplateChild<gtk::ToggleButton>,

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
impl GeneralSettings {
    pub fn set_sender(&self, sender: Sender<Action>) {
        self.sender.set(sender.clone()).unwrap();
    }

    pub fn set_device_state(&self, state: &DeviceState) {
        let Some(sound_modes) = state.sound_modes else {
            return;
        };
        let obj = self.obj();
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

        self.set_ambient_sound_mode(sound_modes.ambient_sound_mode);
        self.set_transparency_mode(sound_modes.transparency_mode);
        self.set_noise_canceling_mode(sound_modes.noise_canceling_mode);
        self.set_custom_noise_canceling(sound_modes.custom_noise_canceling);
    }

    fn set_ambient_sound_mode(&self, ambient_sound_mode: AmbientSoundMode) {
        self.ignore_button_clicks.replace(true);
        let button = match ambient_sound_mode {
            AmbientSoundMode::NoiseCanceling => &self.noise_canceling_mode,
            AmbientSoundMode::Transparency => &self.transparency_mode,
            AmbientSoundMode::Normal => &self.normal_mode,
        };
        button.set_active(true);
        self.ignore_button_clicks.replace(false);
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
    fn handle_normal_mode_clicked(&self, button: &gtk::ToggleButton) {
        if button.is_active() && !self.ignore_button_clicks.get() {
            self.send_action(Action::SetAmbientSoundMode(AmbientSoundMode::Normal));
        }
    }

    #[template_callback]
    fn handle_transparency_mode_clicked(&self, button: &gtk::ToggleButton) {
        if button.is_active() && !self.ignore_button_clicks.get() {
            self.send_action(Action::SetAmbientSoundMode(AmbientSoundMode::Transparency));
        }
    }

    #[template_callback]
    fn handle_noise_canceling_mode_clicked(&self, button: &gtk::ToggleButton) {
        if button.is_active() && !self.ignore_button_clicks.get() {
            self.send_action(Action::SetAmbientSoundMode(
                AmbientSoundMode::NoiseCanceling,
            ));
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
impl ObjectSubclass for GeneralSettings {
    const NAME: &'static str = "OpenSCQ30GeneralSettings";
    type Type = super::GeneralSettings;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for GeneralSettings {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        obj.bind_property(
            "has_noise_canceling_mode",
            &self.noise_canceling_mode_label.get(),
            "visible",
        )
        .sync_create()
        .build();
        [
            &self.noise_canceling_mode.get(),
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
impl WidgetImpl for GeneralSettings {}
impl BoxImpl for GeneralSettings {}

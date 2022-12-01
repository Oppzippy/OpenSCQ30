use gtk::prelude::{ObjectExt, StaticType};
use gtk::subclass::prelude::ObjectSubclassExt;
use gtk::subclass::widget::CompositeTemplateCallbacksClass;
use gtk::{
    glib::{self, once_cell::sync::Lazy, subclass::Signal},
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass},
        widget::{CompositeTemplateClass, WidgetClassSubclassExt, WidgetImpl},
    },
    traits::ToggleButtonExt,
    CompositeTemplate, TemplateChild,
};
use openscq30_lib::packets::structures::ambient_sound_mode::AmbientSoundMode;
use openscq30_lib::packets::structures::noise_canceling_mode::NoiseCancelingMode;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/general_settings.ui")]
pub struct GeneralSettings {
    // Ambient Sound Mode
    #[template_child]
    pub normal_mode: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub transparency_mode: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub noise_canceling_mode: TemplateChild<gtk::ToggleButton>,

    // Noise Canceling Mode
    #[template_child]
    pub transport_mode: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub indoor_mode: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub outdoor_mode: TemplateChild<gtk::ToggleButton>,
}

#[gtk::template_callbacks]
impl GeneralSettings {
    #[template_callback]
    fn handle_normal_mode_clicked(&self, button: &gtk::ToggleButton) {
        if button.is_active() {
            self.obj().emit_by_name(
                "ambient-sound-mode-selected",
                &[&AmbientSoundMode::Normal.id()],
            )
        }
    }

    #[template_callback]
    fn handle_transparency_mode_clicked(&self, button: &gtk::ToggleButton) {
        if button.is_active() {
            self.obj().emit_by_name(
                "ambient-sound-mode-selected",
                &[&AmbientSoundMode::Transparency.id()],
            )
        }
    }

    #[template_callback]
    fn handle_noise_canceling_mode_clicked(&self, button: &gtk::ToggleButton) {
        if button.is_active() {
            self.obj().emit_by_name(
                "ambient-sound-mode-selected",
                &[&AmbientSoundMode::NoiseCanceling.id()],
            )
        }
    }

    #[template_callback]
    fn handle_transport_mode_clicked(&self, button: &gtk::ToggleButton) {
        if button.is_active() {
            self.obj().emit_by_name(
                "noise-canceling-mode-selected",
                &[&NoiseCancelingMode::Transport.id()],
            )
        }
    }

    #[template_callback]
    fn handle_indoor_mode_clicked(&self, button: &gtk::ToggleButton) {
        if button.is_active() {
            self.obj().emit_by_name(
                "noise-canceling-mode-selected",
                &[&NoiseCancelingMode::Indoor.id()],
            )
        }
    }

    #[template_callback]
    fn handle_outdoor_mode_clicked(&self, button: &gtk::ToggleButton) {
        if button.is_active() {
            self.obj().emit_by_name(
                "noise-canceling-mode-selected",
                &[&NoiseCancelingMode::Outdoor.id()],
            )
        }
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
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("ambient-sound-mode-selected")
                    .param_types([u8::static_type()])
                    .build(),
                Signal::builder("noise-canceling-mode-selected")
                    .param_types([u8::static_type()])
                    .build(),
            ]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for GeneralSettings {}
impl BoxImpl for GeneralSettings {}

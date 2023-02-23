use std::rc::Rc;

use gtk::{
    glib::{self, once_cell::sync::Lazy, subclass::Signal},
    prelude::{ObjectExt, StaticType},
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass, ObjectSubclassExt},
        widget::{
            CompositeTemplateCallbacksClass, CompositeTemplateClass,
            CompositeTemplateInitializingExt, WidgetImpl,
        },
    },
    CompositeTemplate, TemplateChild,
};

use gtk::subclass::widget::WidgetClassSubclassExt;
use once_cell::sync::OnceCell;

use crate::{
    objects::EqualizerCustomProfileObject,
    settings::SettingsFile,
    widgets::{EqualizerSettings, GeneralSettings},
};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/selected_device_settings/template.ui")]
pub struct SelectedDeviceSettings {
    #[template_child]
    pub notebook: TemplateChild<gtk::Notebook>,
    #[template_child]
    pub general_settings: TemplateChild<GeneralSettings>,
    #[template_child]
    pub equalizer_settings: TemplateChild<EqualizerSettings>,

    pub settings_file: OnceCell<Rc<SettingsFile>>,
}

#[gtk::template_callbacks]
impl SelectedDeviceSettings {
    #[template_callback]
    fn handle_apply_equalizer_settings(&self, _button: &EqualizerSettings) {
        self.obj().emit_by_name("apply-equalizer-settings", &[])
    }

    #[template_callback]
    // no idea why the parameter comes before &GeneralSettings
    fn handle_ambient_sound_mode_selected(&self, mode: u8, _: &GeneralSettings) {
        let obj = self.obj();
        obj.emit_by_name("ambient-sound-mode-selected", &[&mode])
    }

    #[template_callback]
    fn handle_noise_canceling_mode_selected(&self, mode: u8, _: &GeneralSettings) {
        let obj = self.obj();
        obj.emit_by_name("noise-canceling-mode-selected", &[&mode])
    }

    #[template_callback]
    fn handle_custom_equalizer_profile_selected(&self, profile: &EqualizerCustomProfileObject) {
        self.obj()
            .emit_by_name("custom-equalizer-profile-selected", &[profile])
    }

    #[template_callback]
    fn handle_create_custom_equalizer_profile(&self, _profile: &EqualizerCustomProfileObject) {
        self.obj()
            .emit_by_name("create-custom-equalizer-profile", &[])
    }

    #[template_callback]
    fn handle_delete_custom_equalizer_profile(&self, profile: &EqualizerCustomProfileObject) {
        self.obj()
            .emit_by_name("delete-custom-equalizer-profile", &[&profile])
    }

    #[template_callback]
    fn handle_disconnect(&self, _: &GeneralSettings) {
        self.obj().emit_by_name("disconnect", &[])
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SelectedDeviceSettings {
    const NAME: &'static str = "OpenSCQ30SelectedDeviceSettings";
    type Type = super::SelectedDeviceSettings;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SelectedDeviceSettings {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("ambient-sound-mode-selected")
                    .param_types([u8::static_type()])
                    .build(),
                Signal::builder("noise-canceling-mode-selected")
                    .param_types([u8::static_type()])
                    .build(),
                Signal::builder("apply-equalizer-settings").build(),
                Signal::builder("custom-equalizer-profile-selected")
                    .param_types([EqualizerCustomProfileObject::static_type()])
                    .build(),
                Signal::builder("create-custom-equalizer-profile").build(),
                Signal::builder("delete-custom-equalizer-profile")
                    .param_types([EqualizerCustomProfileObject::static_type()])
                    .build(),
                Signal::builder("disconnect").build(),
            ]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for SelectedDeviceSettings {}
impl BoxImpl for SelectedDeviceSettings {}

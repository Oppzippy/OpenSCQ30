use std::cell::OnceCell;

use gtk::{
    glib::{self, once_cell::sync::Lazy, subclass::Signal, Sender},
    prelude::ObjectExt,
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

use crate::{
    actions::Action,
    objects::CustomEqualizerProfileObject,
    widgets::{EqualizerSettings, GeneralSettings},
};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/OpenSCQ30/selected_device_settings/template.ui")]
pub struct SelectedDeviceSettings {
    #[template_child]
    pub notebook: TemplateChild<gtk::Notebook>,
    #[template_child]
    pub general_settings: TemplateChild<GeneralSettings>,
    #[template_child]
    pub equalizer_settings: TemplateChild<EqualizerSettings>,

    sender: OnceCell<Sender<Action>>,
}

impl SelectedDeviceSettings {
    pub fn set_sender(&self, sender: Sender<Action>) {
        self.sender.set(sender.clone()).unwrap();
        self.general_settings.set_sender(sender.clone());
        self.equalizer_settings.set_sender(sender.clone());
    }
}

#[gtk::template_callbacks]
impl SelectedDeviceSettings {
    #[template_callback]
    fn handle_create_custom_equalizer_profile(&self, _profile: &CustomEqualizerProfileObject) {
        self.obj()
            .emit_by_name("create-custom-equalizer-profile", &[])
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
        static SIGNALS: Lazy<Vec<Signal>> =
            Lazy::new(|| vec![Signal::builder("create-custom-equalizer-profile").build()]);
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for SelectedDeviceSettings {}
impl BoxImpl for SelectedDeviceSettings {}

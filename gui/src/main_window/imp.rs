use gtk::{
    glib::{self, once_cell::sync::Lazy, subclass::Signal},
    prelude::{InitializingWidgetExt, ObjectExt, StaticType},
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass, ObjectSubclassExt},
        widget::{CompositeTemplateCallbacksClass, CompositeTemplateClass, WidgetImpl},
    },
    CompositeTemplate, TemplateChild,
};

use crate::{
    device_selection::{Device, DeviceSelection},
    equalizer::Equalizer,
    general_settings::GeneralSettings,
};
use gtk::subclass::widget::WidgetClassSubclassExt;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/main_window.ui")]
pub struct MainWindow {
    #[template_child]
    pub device_selection: TemplateChild<DeviceSelection>,
    #[template_child]
    pub general_settings: TemplateChild<GeneralSettings>,
    #[template_child]
    pub equalizer: TemplateChild<Equalizer>,
}

#[gtk::template_callbacks]
impl MainWindow {
    pub fn set_devices(&self, devices: &[Device]) {
        self.device_selection.set_devices(devices);
    }

    #[template_callback]
    fn handle_refresh_devices(&self, _device_selection: &DeviceSelection) {
        self.obj().emit_by_name("refresh-devices", &[])
    }

    #[template_callback]
    fn handle_device_selection_changed(&self, _device_selection: &DeviceSelection) {
        self.obj().emit_by_name("device-selection-changed", &[])
    }

    #[template_callback]
    fn handle_apply_custom_equalizer(&self, _button: &gtk::Button) {
        self.obj().emit_by_name("apply-custom-equalizer", &[])
    }

    #[template_callback]
    fn handle_refresh_custom_equalizer(&self, _button: &gtk::Button) {
        self.obj().emit_by_name("refresh-custom-equalizer", &[])
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
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "OpenSCQ30MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("ambient-sound-mode-selected")
                    .param_types([u8::static_type()])
                    .build(),
                Signal::builder("noise-canceling-mode-selected")
                    .param_types([u8::static_type()])
                    .build(),
                Signal::builder("refresh-devices").build(),
                Signal::builder("device-selection-changed").build(),
                Signal::builder("apply-custom-equalizer").build(),
                Signal::builder("refresh-custom-equalizer").build(),
            ]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for MainWindow {}
impl BoxImpl for MainWindow {}

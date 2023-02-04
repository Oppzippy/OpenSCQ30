use std::rc::Rc;

use gtk::{
    glib::{self, once_cell::sync::Lazy, subclass::Signal},
    prelude::{InitializingWidgetExt, ObjectExt, StaticType},
    subclass::{
        prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt},
        widget::{CompositeTemplateCallbacksClass, CompositeTemplateClass, WidgetImpl},
        window::WindowImpl,
    },
    traits::GtkWindowExt,
    CompositeTemplate, Inhibit, TemplateChild,
};

use gtk::subclass::widget::WidgetClassSubclassExt;
use once_cell::sync::OnceCell;

use crate::{
    settings::settings_file::SettingsFile,
    widgets::{Device, DeviceSelection, EqualizerSettings, GeneralSettings},
};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/main_window/template.ui")]
pub struct MainWindow {
    #[template_child]
    pub device_selection: TemplateChild<DeviceSelection>,
    #[template_child]
    pub general_settings: TemplateChild<GeneralSettings>,
    #[template_child]
    pub equalizer_settings: TemplateChild<EqualizerSettings>,

    pub settings_file: OnceCell<Rc<SettingsFile>>,
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
    fn handle_apply_equalizer_settings(&self, _button: &EqualizerSettings) {
        self.obj().emit_by_name("apply-equalizer-settings", &[])
    }

    #[template_callback]
    fn handle_refresh_equalizer_settings(&self, _button: &EqualizerSettings) {
        self.obj().emit_by_name("refresh-equalizer-settings", &[])
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
    type ParentType = gtk::Window;

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
                Signal::builder("apply-equalizer-settings").build(),
                Signal::builder("refresh-equalizer-settings").build(),
            ]
        });
        SIGNALS.as_ref()
    }

    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_title(Some("OpenSCQ30"));
    }
}
impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {
    fn close_request(&self) -> Inhibit {
        self.obj()
            .save_window_size()
            .expect("failed to save window size");
        Inhibit(false)
    }
}

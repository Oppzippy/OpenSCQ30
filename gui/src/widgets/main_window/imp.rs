use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use gtk::{
    glib::{self, clone, once_cell::sync::Lazy, subclass::Signal, ParamSpec, Properties, Value},
    prelude::*,
    subclass::{
        prelude::*,
        widget::{
            CompositeTemplateCallbacksClass, CompositeTemplateClass,
            CompositeTemplateInitializingExt, WidgetImpl,
        },
        window::WindowImpl,
    },
    traits::{BoxExt, DialogExt, EditableExt, GtkWindowExt, WidgetExt},
    CompositeTemplate, DialogFlags, Inhibit, ResponseType, TemplateChild,
};

use gtk::subclass::widget::WidgetClassSubclassExt;
use once_cell::sync::OnceCell;

use crate::{
    objects::{CustomEqualizerProfileObject, DeviceObject},
    settings::Settings,
    widgets::{Device, DeviceSelection, LoadingScreen, SelectedDeviceSettings},
};

#[derive(Default, CompositeTemplate, Properties)]
#[properties(wrapper_type=super::MainWindow)]
#[template(resource = "/com/oppzippy/OpenSCQ30/main_window/template.ui")]
pub struct MainWindow {
    #[template_child]
    pub stack: TemplateChild<gtk::Stack>,
    #[template_child]
    pub selected_device_settings: TemplateChild<SelectedDeviceSettings>,
    #[template_child]
    pub device_selection: TemplateChild<DeviceSelection>,
    #[template_child]
    pub loading_screen: TemplateChild<LoadingScreen>,
    #[template_child]
    pub toast_overlay: TemplateChild<adw::ToastOverlay>,

    #[property(get, set)]
    pub selected_device: RefCell<Option<DeviceObject>>,
    #[property(get, set)]
    pub loading: Cell<bool>,

    pub settings: OnceCell<Rc<Settings>>,
}

#[gtk::template_callbacks]
impl MainWindow {
    pub fn set_devices(&self, devices: &[Device]) {
        self.device_selection.set_devices(devices);
    }

    #[template_callback]
    fn handle_device_selection_changed(&self, _: &DeviceSelection) {
        self.obj().emit_by_name("device-selection-changed", &[])
    }

    #[template_callback]
    fn handle_apply_equalizer_settings(&self, _: &SelectedDeviceSettings) {
        self.obj().emit_by_name("apply-equalizer-settings", &[])
    }

    #[template_callback]
    // no idea why the parameter comes before &GeneralSettings
    fn handle_ambient_sound_mode_selected(&self, mode: u8, _: &SelectedDeviceSettings) {
        let obj = self.obj();
        obj.emit_by_name("ambient-sound-mode-selected", &[&mode])
    }

    #[template_callback]
    fn handle_noise_canceling_mode_selected(&self, mode: u8, _: &SelectedDeviceSettings) {
        let obj = self.obj();
        obj.emit_by_name("noise-canceling-mode-selected", &[&mode])
    }

    #[template_callback]
    fn handle_custom_equalizer_profile_selected(
        &self,
        profile: &CustomEqualizerProfileObject,
        _: &SelectedDeviceSettings,
    ) {
        self.obj()
            .emit_by_name("custom-equalizer-profile-selected", &[profile])
    }

    #[template_callback]
    fn handle_create_custom_equalizer_profile(&self, _: &SelectedDeviceSettings) {
        let obj = self.obj();
        let dialog = gtk::Dialog::with_buttons(
            Some("Create Custom Profile"),
            Some(&*obj),
            DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT | DialogFlags::USE_HEADER_BAR,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Create", ResponseType::Accept),
            ],
        );
        dialog.set_default_response(ResponseType::Accept);

        let entry = gtk::Entry::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .placeholder_text("Name")
            .activates_default(true)
            .build();
        dialog.content_area().append(&entry);

        let accept_button = dialog
            .widget_for_response(ResponseType::Accept)
            .expect("missing accept button");
        accept_button.set_sensitive(false);

        entry.connect_changed(clone!(@weak dialog => move |entry| {
            let button = dialog.widget_for_response(ResponseType::Accept).expect("missing accept button");
            let is_empty = entry.text().trim().is_empty();
            button.set_sensitive(!is_empty);
        }));

        dialog.connect_response(
            clone!(@weak self as this, @weak entry => move |dialog, response| {
                let name = entry.text().to_string();
                dialog.destroy();
                if response != ResponseType::Accept {
                    return;
                }
                let volume_offsets = this
                    .selected_device_settings
                    .equalizer_configuration()
                    .band_offsets()
                    .volume_offsets();

                let profile_with_name = CustomEqualizerProfileObject::new(&name, volume_offsets);
                this.obj().emit_by_name::<()>("create-custom-equalizer-profile", &[&profile_with_name]);
            }),
        );
        dialog.present();
    }

    #[template_callback]
    fn handle_delete_custom_equalizer_profile(
        &self,
        profile: &CustomEqualizerProfileObject,
        _: &SelectedDeviceSettings,
    ) {
        self.obj()
            .emit_by_name("delete-custom-equalizer-profile", &[&profile])
    }

    #[template_callback]
    fn handle_disconnect(&self, _: &gtk::Widget) {
        let selected_device: Option<DeviceObject> = None;
        self.obj().set_property("selected-device", selected_device);
    }

    fn update(&self) {
        if self.loading.get() {
            self.stack.set_visible_child(&self.loading_screen.get());
        } else if self.selected_device.borrow().is_some() {
            self.stack
                .set_visible_child(&self.selected_device_settings.get());
        } else {
            self.stack.set_visible_child(&self.device_selection.get());
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "OpenSCQ30MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        self.parent_constructed();

        self.obj()
            .bind_property("selected-device", self.obj().as_ref(), "title")
            .transform_to(|_, value: Option<DeviceObject>| match value {
                Some(device) => Some(format!(
                    "OpenSCQ30 - {} ({})",
                    device.name(),
                    device.mac_address()
                )),
                None => Some("OpenSCQ30".to_string()),
            })
            .sync_create()
            .build();

        self.device_selection
            .bind_property("selected-device", self.obj().as_ref(), "selected-device")
            .sync_create()
            .build();

        self.obj().connect_notify_local(
            Some("selected-device"),
            clone!(@weak self as this => move |_, _| this.update()),
        );
        self.obj().connect_notify_local(
            Some("loading"),
            clone!(@weak self as this => move |_, _| this.update()),
        );
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("ambient-sound-mode-selected")
                    .param_types([u8::static_type()])
                    .build(),
                Signal::builder("noise-canceling-mode-selected")
                    .param_types([u8::static_type()])
                    .build(),
                Signal::builder("device-selection-changed").build(),
                Signal::builder("apply-equalizer-settings").build(),
                Signal::builder("custom-equalizer-profile-selected")
                    .param_types([CustomEqualizerProfileObject::static_type()])
                    .build(),
                Signal::builder("create-custom-equalizer-profile")
                    .param_types([CustomEqualizerProfileObject::static_type()])
                    .build(),
                Signal::builder("delete-custom-equalizer-profile")
                    .param_types([CustomEqualizerProfileObject::static_type()])
                    .build(),
            ]
        });
        SIGNALS.as_ref()
    }

    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        Self::derived_set_property(self, id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        Self::derived_property(self, id, pspec)
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

impl ApplicationWindowImpl for MainWindow {}

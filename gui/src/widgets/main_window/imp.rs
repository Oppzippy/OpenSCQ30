use std::{
    cell::{Cell, OnceCell, RefCell},
    rc::Rc,
};

use gtk::{
    gio::SimpleAction,
    glib::{self, clone, ParamSpec, Properties, Sender, Value},
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

use crate::{
    actions::Action,
    objects::{BoxedVolumeAdjustments, CustomEqualizerProfileObject, DeviceObject},
    settings::Settings,
    widgets::{DeviceSelection, LoadingScreen, SelectedDeviceSettings},
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
    sender: OnceCell<Sender<Action>>,
}

#[gtk::template_callbacks]
impl MainWindow {
    pub fn set_sender(&self, sender: Sender<Action>) {
        self.sender.set(sender.clone()).unwrap();
        self.loading_screen.set_sender(sender.clone());
        self.device_selection.set_sender(sender.clone());
        self.selected_device_settings.set_sender(sender.clone());
    }

    pub fn set_devices(&self, devices: &[DeviceObject]) {
        self.device_selection.set_devices(devices);
    }

    fn create_custom_equalizer_profile(&self, volume_adjustments: [f64; 8]) {
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
            clone!(@weak self as this, @weak entry, @strong volume_adjustments => move |dialog, response| {
                let name = entry.text().to_string();
                dialog.destroy();
                if response != ResponseType::Accept {
                    return;
                }

                let profile_with_name = CustomEqualizerProfileObject::new(&name, volume_adjustments);
                this.sender.get().unwrap().send(Action::CreateCustomEqualizerProfile(profile_with_name)).unwrap();
            }),
        );
        dialog.present();
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

        let action = SimpleAction::new(
            "create-custom-equalizer-profile",
            Some(&BoxedVolumeAdjustments::static_variant_type()),
        );
        action.connect_activate(
            clone!(@weak self as this => move |_action, parameter| {
                let boxed_volume_adjustments: BoxedVolumeAdjustments = parameter.unwrap().get().unwrap();
                let volume_adjustments = boxed_volume_adjustments.0
                    .iter()
                    .map(ToOwned::to_owned)
                    .collect::<Vec<f64>>()
                    .try_into()
                    .unwrap();
                this.create_custom_equalizer_profile(volume_adjustments);
            }),
        );
        self.obj().add_action(&action);

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

        self.obj().connect_notify_local(
            Some("selected-device"),
            clone!(@weak self as this => move |_, _| this.update()),
        );
        self.obj().connect_notify_local(
            Some("loading"),
            clone!(@weak self as this => move |_, _| this.update()),
        );
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

use std::cell::RefCell;

use gtk::{
    gio,
    glib::{self, ParamSpec, Properties, Value},
    prelude::*,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass, *},
        widget::{
            CompositeTemplateCallbacksClass, CompositeTemplateClass,
            CompositeTemplateInitializingExt, WidgetClassSubclassExt, WidgetImpl,
        },
    },
    ClosureExpression, CompositeTemplate, TemplateChild,
};
use once_cell::unsync::OnceCell;

use crate::objects::DeviceObject;

use super::Device;

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/com/oppzippy/OpenSCQ30/device_selection/template.ui")]
#[properties(wrapper_type=super::DeviceSelection)]
pub struct DeviceSelection {
    #[template_child]
    pub dropdown: TemplateChild<gtk::DropDown>,

    #[property(get, set)]
    pub selected_device: RefCell<Option<DeviceObject>>,

    pub devices: OnceCell<gio::ListStore>,
}

#[gtk::template_callbacks]
impl DeviceSelection {
    #[template_callback]
    pub fn handle_connect_clicked(&self, _button: &gtk::Button) {
        let selected_device: Option<DeviceObject> = self.dropdown.selected_item().and_downcast();
        // `self.obj().set_selected_device()` from derive(Properties) doesn't allow None
        self.obj().set_property("selected-device", selected_device);
    }

    pub fn set_devices(&self, devices: &[Device]) {
        let objects = devices
            .iter()
            .map(|device| DeviceObject::new(&device.name, &device.mac_address))
            .collect::<Vec<_>>();

        if let Some(model) = self.devices.get() {
            model.remove_all();
            model.extend_from_slice(&objects);

            self.dropdown.set_model(Some(model));
        }
    }

    pub fn selected_device(&self) -> Option<Device> {
        self.dropdown
            .selected_item()
            .map(|object| {
                object
                    .downcast::<DeviceObject>()
                    .expect("selected item must be a DeviceObject")
            })
            .map(|device_object| Device {
                name: device_object.name(),
                mac_address: device_object.mac_address(),
            })
    }
}

#[glib::object_subclass]
impl ObjectSubclass for DeviceSelection {
    const NAME: &'static str = "OpenSCQ30DeviceSelection";
    type Type = super::DeviceSelection;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for DeviceSelection {
    fn constructed(&self) {
        self.parent_constructed();
        let model = gio::ListStore::new(DeviceObject::static_type());
        self.dropdown.set_model(Some(&model));
        self.devices
            .set(model)
            .expect("constructed should only run once");

        let expression = ClosureExpression::with_callback(gtk::Expression::NONE, |args| {
            let device_object: DeviceObject = args[0].get().unwrap();
            format!(
                "{}: [{}]",
                device_object.name(),
                device_object.mac_address()
            )
        });
        self.dropdown.set_expression(Some(expression));
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
impl WidgetImpl for DeviceSelection {}
impl BoxImpl for DeviceSelection {}

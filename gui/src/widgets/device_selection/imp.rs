use gtk::glib::clone;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::prelude::{Cast, ObjectExt, StaticType};
use gtk::subclass::prelude::{ObjectImplExt, ObjectSubclassExt};
use gtk::subclass::widget::{CompositeTemplateCallbacksClass, CompositeTemplateInitializingExt};
use gtk::{gio, ClosureExpression};
use gtk::{
    glib,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass},
        widget::{CompositeTemplateClass, WidgetClassSubclassExt, WidgetImpl},
    },
    CompositeTemplate, TemplateChild,
};
use once_cell::unsync::OnceCell;

use crate::objects::DeviceObject;

use super::Device;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/device_selection/template.ui")]
pub struct DeviceSelection {
    #[template_child]
    pub dropdown: TemplateChild<gtk::DropDown>,

    pub devices: OnceCell<gio::ListStore>,
}

#[gtk::template_callbacks]
impl DeviceSelection {
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

    #[template_callback]
    fn handle_refresh_clicked(&self, _button: &gtk::Button) {
        self.obj().emit_by_name("refresh-devices", &[])
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

        self.dropdown
            .connect_selected_item_notify(clone!(@weak self as this => move |_dropdown| {
                this.obj().emit_by_name("selection-changed", &[])
            }));
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("refresh-devices").build(),
                Signal::builder("selection-changed").build(),
            ]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for DeviceSelection {}
impl BoxImpl for DeviceSelection {}

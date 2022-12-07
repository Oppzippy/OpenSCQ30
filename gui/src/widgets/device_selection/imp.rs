use std::cell::RefCell;

use gtk::gio;
use gtk::glib::clone;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::prelude::{Cast, ObjectExt, StaticType};
use gtk::subclass::prelude::ObjectSubclassExt;
use gtk::subclass::widget::CompositeTemplateCallbacksClass;
use gtk::{
    glib,
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass},
        widget::{CompositeTemplateClass, WidgetClassSubclassExt, WidgetImpl},
    },
    CompositeTemplate, TemplateChild,
};
use gtk::{SignalListItemFactory, SingleSelection};

use crate::objects::DeviceObject;

use super::Device;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/device_selection/template.ui")]
pub struct DeviceSelection {
    #[template_child]
    pub dropdown: TemplateChild<gtk::DropDown>,

    pub devices: RefCell<Option<gio::ListStore>>,
}

#[gtk::template_callbacks]
impl DeviceSelection {
    pub fn set_devices(&self, devices: &[Device]) {
        let objects: Vec<DeviceObject> = devices
            .iter()
            .map(|device| DeviceObject::new(&device.name, &device.mac_address))
            .collect();

        if let Some(model) = &*self.devices.borrow() {
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
        let model = gio::ListStore::new(DeviceObject::static_type());
        self.devices.replace(Some(model));

        let selection_model = SingleSelection::new(self.devices.borrow().to_owned().as_ref());
        self.dropdown.set_model(Some(&selection_model));

        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = gtk::Label::new(None);
            list_item.set_child(Some(&label));
        });

        factory.connect_bind(move |_, list_item| {
            let device_object = list_item
                .item()
                .expect("item must exist")
                .downcast::<DeviceObject>()
                .expect("the item must be a DeviceObject");

            let label = list_item
                .child()
                .expect("must have a child")
                .downcast::<gtk::Label>()
                .expect("child must be a Label");

            let name = device_object.name();
            let mac_address = device_object.mac_address();

            label.set_label(&format!("{name}: [{mac_address}]"));
        });

        let obj = self.obj();
        self.dropdown
            .connect_selected_item_notify(clone!(@weak obj => move |_dropdown| {
                obj.emit_by_name("selection-changed", &[])
            }));

        self.dropdown.set_factory(Some(&factory));
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

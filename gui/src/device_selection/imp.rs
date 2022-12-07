use std::borrow::Borrow;
use std::cell::RefCell;

use gtk::gio::{self, ListStore};
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::glib::BindingFlags;
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
use gtk::{NoSelection, SignalListItemFactory, SingleSelection};

use crate::device_object::DeviceObject;

use super::Device;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/device_selection.ui")]
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
        let model = ListStore::new(DeviceObject::static_type());
        model.extend_from_slice(&objects);

        self.dropdown.set_model(Some(&model));
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

            let name = device_object.property::<String>("name");
            let mac_address = device_object.property::<String>("mac-address");

            label.set_label(&format!("{name}: [{mac_address}]"));
        });

        self.dropdown.set_factory(Some(&factory));
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> =
            Lazy::new(|| vec![Signal::builder("refresh-devices").build()]);
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for DeviceSelection {}
impl BoxImpl for DeviceSelection {}

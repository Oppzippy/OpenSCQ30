use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{actions::Action, objects::GlibDevice};

glib::wrapper! {
    pub struct DeviceSelection(ObjectSubclass<imp::DeviceSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DeviceSelection {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: UnboundedSender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_devices(&self, devices: &[GlibDevice]) {
        self.imp().set_devices(devices)
    }
}

mod imp {
    use std::{
        cell::{OnceCell, RefCell},
        str::FromStr,
    };

    use gtk::{
        gio,
        glib::{self, ParamSpec, Properties, Value},
        prelude::*,
        subclass::{
            prelude::{BoxImpl, ObjectImpl, ObjectSubclass, *},
            widget::{
                CompositeTemplateCallbacksClass, CompositeTemplateClass,
                CompositeTemplateInitializingExt, WidgetImpl,
            },
        },
        ClosureExpression, CompositeTemplate, TemplateChild,
    };
    use macaddr::MacAddr6;
    use tokio::sync::mpsc::UnboundedSender;

    use crate::{actions::Action, objects::GlibDevice};

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/device_selection.ui")]
    #[properties(wrapper_type=super::DeviceSelection)]
    pub struct DeviceSelection {
        #[template_child]
        pub dropdown: TemplateChild<gtk::DropDown>,

        #[property(get, set)]
        pub selected_device: RefCell<Option<GlibDevice>>,

        pub devices: OnceCell<gio::ListStore>,
        sender: OnceCell<UnboundedSender<Action>>,
    }

    #[gtk::template_callbacks]
    impl DeviceSelection {
        pub fn set_sender(&self, sender: UnboundedSender<Action>) {
            self.sender.set(sender).unwrap();
        }

        #[template_callback]
        pub fn handle_connect_clicked(&self, _button: &gtk::Button) {
            if let Some(selected_device) =
                self.dropdown.selected_item().and_downcast::<GlibDevice>()
            {
                self.sender
                    .get()
                    .unwrap()
                    .send(Action::Connect(
                        MacAddr6::from_str(&selected_device.mac_address()).unwrap(),
                    ))
                    .unwrap();
            }
        }

        pub fn set_devices(&self, devices: &[GlibDevice]) {
            if let Some(model) = self.devices.get() {
                model.remove_all();
                model.extend_from_slice(devices);

                self.dropdown.set_model(Some(model));
            }
        }

        pub fn selected_device(&self) -> Option<GlibDevice> {
            self.dropdown.selected_item().map(|object| {
                object
                    .downcast::<GlibDevice>()
                    .expect("selected item must be a DeviceObject")
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
            let model = gio::ListStore::new::<GlibDevice>();
            self.dropdown.set_model(Some(&model));
            self.devices
                .set(model)
                .expect("constructed should only run once");

            let expression = ClosureExpression::with_callback(gtk::Expression::NONE, |args| {
                let device_object: GlibDevice = args[0].get().unwrap();
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
}

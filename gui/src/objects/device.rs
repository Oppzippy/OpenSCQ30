use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct GlibDevice(ObjectSubclass<imp::GlibDevice>);
}

impl GlibDevice {
    pub fn new(name: &str, mac_address: &str) -> Self {
        let obj: Self = Object::builder()
            .property("name", name)
            .property("mac-address", mac_address)
            .build();
        obj
    }
}

mod imp {
    use std::cell::RefCell;

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::GlibDevice)]
    pub struct GlibDevice {
        #[property(set, get)]
        pub name: RefCell<String>,
        #[property(set, get)]
        pub mac_address: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GlibDevice {
        const NAME: &'static str = "OpenSCQ30ObjectsDevice";
        type Type = super::GlibDevice;
    }

    impl ObjectImpl for GlibDevice {
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
}

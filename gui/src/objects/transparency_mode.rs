use gtk::glib::{self, Object};
use openscq30_lib::packets::structures::TransparencyMode;

glib::wrapper! {
    pub struct GlibTransparencyMode(ObjectSubclass<imp::GlibTransparencyMode>);
}

impl GlibTransparencyMode {
    pub fn new(transparency_mode: GlibTransparencyModeValue, name: &str) -> Self {
        Object::builder()
            .property("transparency-mode", transparency_mode)
            .property("name", name)
            .build()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30BoxedTransparencyMode")]
pub struct GlibTransparencyModeValue(pub TransparencyMode);

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    use crate::objects::GlibTransparencyModeValue;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::GlibTransparencyMode)]
    pub struct GlibTransparencyMode {
        #[property(set, get)]
        pub transparency_mode: Cell<GlibTransparencyModeValue>,
        #[property(set, get)]
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GlibTransparencyMode {
        const NAME: &'static str = "OpenSCQ30ObjectsTransparencyMode";
        type Type = super::GlibTransparencyMode;
    }

    impl ObjectImpl for GlibTransparencyMode {
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

use gtk::glib::{self, Object};

use crate::objects::BoxedTransparencyMode;

glib::wrapper! {
    pub struct TransparencyModeModel(ObjectSubclass<imp::TransparencyModeModel>);
}

impl TransparencyModeModel {
    pub fn new(transparency_mode: BoxedTransparencyMode, name: &str) -> Self {
        Object::builder()
            .property("transparency-mode", transparency_mode)
            .property("name", name)
            .build()
    }
}

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    use crate::objects::BoxedTransparencyMode;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::TransparencyModeModel)]
    pub struct TransparencyModeModel {
        #[property(set, get)]
        pub transparency_mode: Cell<BoxedTransparencyMode>,
        #[property(set, get)]
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TransparencyModeModel {
        const NAME: &'static str = "OpenSCQ30TransparencyModeModel";
        type Type = super::TransparencyModeModel;
    }

    impl ObjectImpl for TransparencyModeModel {
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

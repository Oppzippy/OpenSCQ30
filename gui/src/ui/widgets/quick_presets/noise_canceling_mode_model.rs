use gtk::glib::{self, Object};

use crate::objects::BoxedNoiseCancelingMode;

glib::wrapper! {
    pub struct NoiseCancelingModeModel(ObjectSubclass<imp::NoiseCancelingModeModel>);
}

impl NoiseCancelingModeModel {
    pub fn new(noise_canceling_mode: BoxedNoiseCancelingMode, name: &str) -> Self {
        Object::builder()
            .property("noise-canceling-mode", noise_canceling_mode)
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

    use crate::objects::BoxedNoiseCancelingMode;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::NoiseCancelingModeModel)]
    pub struct NoiseCancelingModeModel {
        #[property(set, get)]
        pub noise_canceling_mode: Cell<BoxedNoiseCancelingMode>,
        #[property(set, get)]
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NoiseCancelingModeModel {
        const NAME: &'static str = "OpenSCQ30NoiseCancelingModeModel";
        type Type = super::NoiseCancelingModeModel;
    }

    impl ObjectImpl for NoiseCancelingModeModel {
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

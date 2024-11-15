use gtk::glib::{self, Object};
use openscq30_lib::devices::standard::structures::NoiseCancelingModeTypeTwo;

glib::wrapper! {
    pub struct GlibNoiseCancelingModeTypeTwo(ObjectSubclass<imp::GlibNoiseCancelingModeTypeTwo>);
}

impl GlibNoiseCancelingModeTypeTwo {
    pub fn new(noise_canceling_mode: GlibNoiseCancelingModeTypeTwoValue) -> Self {
        Object::builder()
            .property("noise-canceling-mode", noise_canceling_mode)
            .build()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30BoxedNoiseCancelingModeTypeTwo")]
pub struct GlibNoiseCancelingModeTypeTwoValue(pub NoiseCancelingModeTypeTwo);

mod imp {
    use std::cell::Cell;

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    use super::GlibNoiseCancelingModeTypeTwoValue;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::GlibNoiseCancelingModeTypeTwo)]
    pub struct GlibNoiseCancelingModeTypeTwo {
        #[property(set, get)]
        pub noise_canceling_mode: Cell<GlibNoiseCancelingModeTypeTwoValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GlibNoiseCancelingModeTypeTwo {
        const NAME: &'static str = "OpenSCQ30ObjectsNoiseCancelingModeTypeTwo";
        type Type = super::GlibNoiseCancelingModeTypeTwo;
    }

    impl ObjectImpl for GlibNoiseCancelingModeTypeTwo {
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

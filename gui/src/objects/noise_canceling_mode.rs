use gtk::glib::{self, Object};
use openscq30_lib::packets::structures::NoiseCancelingMode;

glib::wrapper! {
    pub struct GlibNoiseCancelingMode(ObjectSubclass<imp::GlibNoiseCancelingMode>);
}

impl GlibNoiseCancelingMode {
    pub fn new(noise_canceling_mode: GlibNoiseCancelingModeValue) -> Self {
        Object::builder()
            .property("noise-canceling-mode", noise_canceling_mode)
            .build()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30BoxedNoiseCancelingMode")]
pub struct GlibNoiseCancelingModeValue(pub NoiseCancelingMode);

mod imp {
    use std::cell::Cell;

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    use super::GlibNoiseCancelingModeValue;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::GlibNoiseCancelingMode)]
    pub struct GlibNoiseCancelingMode {
        #[property(set, get)]
        pub noise_canceling_mode: Cell<GlibNoiseCancelingModeValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GlibNoiseCancelingMode {
        const NAME: &'static str = "OpenSCQ30ObjectsNoiseCancelingMode";
        type Type = super::GlibNoiseCancelingMode;
    }

    impl ObjectImpl for GlibNoiseCancelingMode {
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

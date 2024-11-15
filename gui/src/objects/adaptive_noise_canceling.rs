use gtk::glib::{self, Object};
use openscq30_lib::devices::standard::structures::AdaptiveNoiseCanceling;

glib::wrapper! {
    pub struct GlibAdaptiveNoiseCanceling(ObjectSubclass<imp::GlibAdaptiveNoiseCanceling>);
}

impl GlibAdaptiveNoiseCanceling {
    pub fn new(adaptive_noise_canceling: GlibAdaptiveNoiseCancelingValue) -> Self {
        Object::builder()
            .property("adaptive-noise-canceling", adaptive_noise_canceling)
            .build()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30BoxedAdaptiveNoiseCanceling")]
pub struct GlibAdaptiveNoiseCancelingValue(pub AdaptiveNoiseCanceling);

mod imp {
    use std::cell::Cell;

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    use super::GlibAdaptiveNoiseCancelingValue;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::GlibAdaptiveNoiseCanceling)]
    pub struct GlibAdaptiveNoiseCanceling {
        #[property(set, get)]
        pub adaptive_noise_canceling: Cell<GlibAdaptiveNoiseCancelingValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GlibAdaptiveNoiseCanceling {
        const NAME: &'static str = "OpenSCQ30ObjectsAdaptiveNoiseCanceling";
        type Type = super::GlibAdaptiveNoiseCanceling;
    }

    impl ObjectImpl for GlibAdaptiveNoiseCanceling {
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

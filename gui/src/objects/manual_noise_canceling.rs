use gtk::glib::{self, Object};
use openscq30_lib::devices::standard::structures::ManualNoiseCanceling;

glib::wrapper! {
    pub struct GlibManualNoiseCanceling(ObjectSubclass<imp::GlibManualNoiseCanceling>);
}

impl GlibManualNoiseCanceling {
    pub fn new(manual_noise_canceling: GlibManualNoiseCancelingValue) -> Self {
        Object::builder()
            .property("manual-noise-canceling", manual_noise_canceling)
            .build()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30BoxedManualNoiseCanceling")]
pub struct GlibManualNoiseCancelingValue(pub ManualNoiseCanceling);

mod imp {
    use std::cell::Cell;

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    use super::GlibManualNoiseCancelingValue;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::GlibManualNoiseCanceling)]
    pub struct GlibManualNoiseCanceling {
        #[property(set, get)]
        pub manual_noise_canceling: Cell<GlibManualNoiseCancelingValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GlibManualNoiseCanceling {
        const NAME: &'static str = "OpenSCQ30ObjectsManualNoiseCanceling";
        type Type = super::GlibManualNoiseCanceling;
    }

    impl ObjectImpl for GlibManualNoiseCanceling {
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

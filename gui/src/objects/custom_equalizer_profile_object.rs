use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

glib::wrapper! {
    pub struct CustomEqualizerProfileObject(ObjectSubclass<imp::CustomEqualizerProfileObject>);
}

impl CustomEqualizerProfileObject {
    pub fn new(name: &str, volume_adjustments: [f64; 8]) -> Self {
        let obj: Self = Object::builder().property("name", name).build();
        obj.imp().volume_adjustments.replace(volume_adjustments);
        obj
    }

    pub fn volume_adjustments(&self) -> [f64; 8] {
        self.imp().volume_adjustments.get()
    }
}

#[derive(Debug, PartialEq, Clone, glib::Boxed, glib::Variant)]
#[boxed_type(name = "OpenSCQ30VolumeAdjustments")]
pub struct BoxedVolumeAdjustments(pub glib::FixedSizeVariantArray<Vec<f64>, f64>);

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::CustomEqualizerProfileObject)]
    pub struct CustomEqualizerProfileObject {
        #[property(get, set)]
        pub name: RefCell<String>,
        pub volume_adjustments: Cell<[f64; 8]>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CustomEqualizerProfileObject {
        const NAME: &'static str = "OpenSCQ30CustomEqualizerProfileObject";
        type Type = super::CustomEqualizerProfileObject;
    }

    impl ObjectImpl for CustomEqualizerProfileObject {
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

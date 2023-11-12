use std::sync::Arc;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

glib::wrapper! {
    pub struct GlibCustomEqualizerProfile(ObjectSubclass<imp::GlibCustomEqualizerProfile>);
}

impl GlibCustomEqualizerProfile {
    pub fn new(name: &str, volume_adjustments: Arc<[f64]>) -> Self {
        let obj: Self = Object::builder().property("name", name).build();
        obj.imp().volume_adjustments.replace(volume_adjustments);
        obj
    }

    pub fn volume_adjustments(&self) -> Arc<[f64]> {
        self.imp().volume_adjustments.borrow().to_owned()
    }
}

#[derive(Debug, PartialEq, Clone, glib::Boxed, glib::Variant)]
#[boxed_type(name = "OpenSCQ30ObjectsVolumeAdjustments")]
pub struct GlibVolumeAdjustments(pub glib::FixedSizeVariantArray<Vec<f64>, f64>);

mod imp {
    use std::{cell::RefCell, sync::Arc};

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };
    use openscq30_lib::devices::standard::structures::VolumeAdjustments;

    #[derive(Properties)]
    #[properties(wrapper_type = super::GlibCustomEqualizerProfile)]
    pub struct GlibCustomEqualizerProfile {
        #[property(get, set)]
        pub name: RefCell<String>,
        pub volume_adjustments: RefCell<Arc<[f64]>>,
    }

    impl Default for GlibCustomEqualizerProfile {
        fn default() -> Self {
            Self {
                name: Default::default(),
                volume_adjustments: RefCell::new(VolumeAdjustments::default().adjustments()),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GlibCustomEqualizerProfile {
        const NAME: &'static str = "OpenSCQ30ObjectsCustomEqualizerProfile";
        type Type = super::GlibCustomEqualizerProfile;
    }

    impl ObjectImpl for GlibCustomEqualizerProfile {
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

use gtk::glib::{self, Object};
use openscq30_lib::devices::standard::structures::AmbientSoundMode;

glib::wrapper! {
    pub struct GlibAmbientSoundMode(ObjectSubclass<imp::GlibAmbientSoundMode>);
}

impl GlibAmbientSoundMode {
    pub fn new(ambient_sound_mode: GlibAmbientSoundModeValue) -> Self {
        Object::builder()
            .property("ambient-sound-mode", ambient_sound_mode)
            .build()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30ValuesAmbientSoundMode")]
pub struct GlibAmbientSoundModeValue(pub AmbientSoundMode);

mod imp {
    use std::cell::Cell;

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    use super::GlibAmbientSoundModeValue;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::GlibAmbientSoundMode)]
    pub struct GlibAmbientSoundMode {
        #[property(set, get)]
        pub ambient_sound_mode: Cell<GlibAmbientSoundModeValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GlibAmbientSoundMode {
        const NAME: &'static str = "OpenSCQ30AmbientSoundModeModel";
        type Type = super::GlibAmbientSoundMode;
    }

    impl ObjectImpl for GlibAmbientSoundMode {
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

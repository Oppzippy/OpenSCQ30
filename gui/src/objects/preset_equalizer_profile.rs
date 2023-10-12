use gtk::glib::{self, Object};
use openscq30_lib::packets::structures::PresetEqualizerProfile;

glib::wrapper! {
    pub struct GlibPresetEqualizerProfile(ObjectSubclass<imp::GlibPresetEqualizerProfile>);
}

impl GlibPresetEqualizerProfile {
    pub fn new(preset_equalizer_profile: GlibPresetEqualizerProfileValue, name: &str) -> Self {
        Object::builder()
            .property("preset-equalizer-profile", preset_equalizer_profile)
            .property("name", name)
            .build()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30ValuesPresetEqualizerProfile")]
pub struct GlibPresetEqualizerProfileValue(pub PresetEqualizerProfile);

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    use super::GlibPresetEqualizerProfileValue;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::GlibPresetEqualizerProfile)]
    pub struct GlibPresetEqualizerProfile {
        #[property(set, get)]
        pub preset_equalizer_profile: Cell<GlibPresetEqualizerProfileValue>,
        #[property(set, get)]
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GlibPresetEqualizerProfile {
        const NAME: &'static str = "OpenSCQ30ObjectsPresetEqualizerProfile";
        type Type = super::GlibPresetEqualizerProfile;
    }

    impl ObjectImpl for GlibPresetEqualizerProfile {
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

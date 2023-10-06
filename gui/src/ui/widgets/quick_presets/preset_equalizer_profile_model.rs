use gtk::glib::{self, Object};

use crate::objects::BoxedPresetEqualizerProfile;

glib::wrapper! {
    pub struct PresetEqualizerProfileModel(ObjectSubclass<imp::PresetEqualizerProfileModel>);
}

impl PresetEqualizerProfileModel {
    pub fn new(preset_equalizer_profile: BoxedPresetEqualizerProfile, name: &str) -> Self {
        Object::builder()
            .property("preset-equalizer-profile", preset_equalizer_profile)
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

    use crate::objects::BoxedPresetEqualizerProfile;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::PresetEqualizerProfileModel)]
    pub struct PresetEqualizerProfileModel {
        #[property(set, get)]
        pub preset_equalizer_profile: Cell<BoxedPresetEqualizerProfile>,
        #[property(set, get)]
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PresetEqualizerProfileModel {
        const NAME: &'static str = "OpenSCQ30PresetEqualizerProfileModel";
        type Type = super::PresetEqualizerProfileModel;
    }

    impl ObjectImpl for PresetEqualizerProfileModel {
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

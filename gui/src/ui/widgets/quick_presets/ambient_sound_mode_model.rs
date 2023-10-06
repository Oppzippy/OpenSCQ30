use gtk::glib::{self, Object};

use crate::objects::BoxedAmbientSoundMode;

glib::wrapper! {
    pub struct AmbientSoundModeModel(ObjectSubclass<imp::AmbientSoundModeModel>);
}

impl AmbientSoundModeModel {
    pub fn new(ambient_sound_mode: BoxedAmbientSoundMode, name: &str) -> Self {
        Object::builder()
            .property("ambient-sound-mode", ambient_sound_mode)
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

    use crate::objects::BoxedAmbientSoundMode;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::AmbientSoundModeModel)]
    pub struct AmbientSoundModeModel {
        #[property(set, get)]
        pub ambient_sound_mode: Cell<BoxedAmbientSoundMode>,
        #[property(set, get)]
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AmbientSoundModeModel {
        const NAME: &'static str = "OpenSCQ30AmbientSoundModeModel";
        type Type = super::AmbientSoundModeModel;
    }

    impl ObjectImpl for AmbientSoundModeModel {
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

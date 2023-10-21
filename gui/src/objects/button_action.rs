use gtk::glib::{self, Object};
use openscq30_lib::packets::structures::ButtonAction;

glib::wrapper! {
    pub struct GlibButtonAction(ObjectSubclass<imp::GlibButtonAction>);
}

impl GlibButtonAction {
    pub fn new(button_action: GlibButtonActionValue) -> Self {
        Object::builder()
            .property("button-action", button_action)
            .build()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, glib::Boxed, Default)]
#[boxed_type(name = "OpenSCQ30ValuesButtonAction")]
pub struct GlibButtonActionValue(pub Option<ButtonAction>);

mod imp {
    use std::cell::Cell;

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::ObjectExt,
        subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
    };

    use super::GlibButtonActionValue;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::GlibButtonAction)]
    pub struct GlibButtonAction {
        #[property(set, get)]
        pub button_action: Cell<GlibButtonActionValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GlibButtonAction {
        const NAME: &'static str = "OpenSCQ30ButtonActionModel";
        type Type = super::GlibButtonAction;
    }

    impl ObjectImpl for GlibButtonAction {
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

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct CustomNoiseCancelingSelection(ObjectSubclass<imp::CustomNoiseCancelingSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomNoiseCancelingSelection {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    // Properties macro creates an enum for internal use. We don't care that it is caught by this lint.
    #![allow(clippy::enum_variant_names)]

    use std::cell::Cell;

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::*,
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        CompositeTemplate, TemplateChild,
    };
    use openscq30_lib::devices::standard::structures::CustomNoiseCanceling;

    use crate::objects::GlibCustomNoiseCancelingValue;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/custom_noise_canceling_selection.ui"
    )]
    #[properties(wrapper_type=super::CustomNoiseCancelingSelection)]
    pub struct CustomNoiseCancelingSelection {
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child]
        pub scale: TemplateChild<gtk::Scale>,

        #[property(set, get)]
        custom_noise_canceling: Cell<GlibCustomNoiseCancelingValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CustomNoiseCancelingSelection {
        const NAME: &'static str = "OpenSCQ30CustomNoiseCancelingSelection";
        type Type = super::CustomNoiseCancelingSelection;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CustomNoiseCancelingSelection {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.bind_property("custom_noise_canceling", &self.scale.adjustment(), "value")
                .transform_to(
                    move |_, custom_noise_canceling: GlibCustomNoiseCancelingValue| {
                        Some(custom_noise_canceling.0.value() as f64)
                    },
                )
                .transform_from(|_, value: f64| {
                    Some(GlibCustomNoiseCancelingValue(CustomNoiseCanceling::new(
                        value as u8,
                    )))
                })
                .sync_create()
                .bidirectional()
                .build();
        }

        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }

    impl WidgetImpl for CustomNoiseCancelingSelection {}
    impl BoxImpl for CustomNoiseCancelingSelection {}
}

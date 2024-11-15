use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct NoiseCancelingSensitivityLevelSelection(ObjectSubclass<imp::NoiseCancelingSensitivityLevelSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl NoiseCancelingSensitivityLevelSelection {
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

    use crate::objects::GlibNoiseCancelingSensitivityLevelValue;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/noise_canceling_sensitivity_level_selection.ui"
    )]
    #[properties(wrapper_type=super::NoiseCancelingSensitivityLevelSelection)]
    pub struct NoiseCancelingSensitivityLevelSelection {
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child]
        pub scale: TemplateChild<gtk::Scale>,

        #[property(set, get)]
        noise_canceling_sensitivity_level: Cell<GlibNoiseCancelingSensitivityLevelValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NoiseCancelingSensitivityLevelSelection {
        const NAME: &'static str = "OpenSCQ30NoiseCancelingSensitivityLevelSelection";
        type Type = super::NoiseCancelingSensitivityLevelSelection;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for NoiseCancelingSensitivityLevelSelection {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.bind_property(
                "noise-canceling-sensitivity-level",
                &self.scale.adjustment(),
                "value",
            )
            .transform_to(
                move |_, custom_noise_canceling: GlibNoiseCancelingSensitivityLevelValue| {
                    Some(custom_noise_canceling.0 as f64)
                },
            )
            .transform_from(|_, value: f64| {
                Some(GlibNoiseCancelingSensitivityLevelValue(value as u8))
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

    impl WidgetImpl for NoiseCancelingSensitivityLevelSelection {}
    impl BoxImpl for NoiseCancelingSensitivityLevelSelection {}
}

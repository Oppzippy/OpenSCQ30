use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct ManualNoiseCancelingSelection(ObjectSubclass<imp::ManualNoiseCancelingSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ManualNoiseCancelingSelection {
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
    use openscq30_lib::devices::standard::structures::ManualNoiseCanceling;

    use crate::objects::GlibManualNoiseCancelingValue;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/manual_noise_canceling_selection.ui"
    )]
    #[properties(wrapper_type=super::ManualNoiseCancelingSelection)]
    pub struct ManualNoiseCancelingSelection {
        #[template_child]
        pub manual_noise_canceling_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub weak: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub moderate: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub strong: TemplateChild<gtk::ToggleButton>,

        #[property(set, get)]
        manual_noise_canceling: Cell<GlibManualNoiseCancelingValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ManualNoiseCancelingSelection {
        const NAME: &'static str = "OpenSCQ30ManualNoiseCancelingSelection";
        type Type = super::ManualNoiseCancelingSelection;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ManualNoiseCancelingSelection {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            [
                (ManualNoiseCanceling::Weak, &self.weak.get()),
                (ManualNoiseCanceling::Moderate, &self.moderate.get()),
                (ManualNoiseCanceling::Strong, &self.strong.get()),
            ]
            .into_iter()
            .for_each(|(button_manual_noise_canceling, button)| {
                obj.bind_property("manual_noise_canceling", button, "active")
                    .transform_to(
                        move |_, selected_manual_noise_canceling: GlibManualNoiseCancelingValue| {
                            Some(button_manual_noise_canceling == selected_manual_noise_canceling.0)
                        },
                    )
                    .transform_from(move |_, is_active| {
                        if is_active {
                            Some(GlibManualNoiseCancelingValue(button_manual_noise_canceling))
                        } else {
                            None
                        }
                    })
                    .sync_create()
                    .bidirectional()
                    .build();
            });
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

    impl WidgetImpl for ManualNoiseCancelingSelection {}
    impl BoxImpl for ManualNoiseCancelingSelection {}
}

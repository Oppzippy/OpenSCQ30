use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct NoiseCancelingModeSelection(ObjectSubclass<imp::NoiseCancelingModeSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl NoiseCancelingModeSelection {
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
    use openscq30_lib::packets::structures::NoiseCancelingMode;

    use crate::objects::GlibNoiseCancelingModeValue;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/noise_canceling_mode_selection.ui"
    )]
    #[properties(wrapper_type=super::NoiseCancelingModeSelection)]
    pub struct NoiseCancelingModeSelection {
        #[template_child]
        pub noise_canceling_mode_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub transport_mode: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub indoor_mode: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub outdoor_mode: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub custom_mode: TemplateChild<gtk::ToggleButton>,

        #[property(set, get)]
        has_custom_noise_canceling: Cell<bool>,
        #[property(set, get)]
        noise_canceling_mode: Cell<GlibNoiseCancelingModeValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NoiseCancelingModeSelection {
        const NAME: &'static str = "OpenSCQ30NoiseCancelingModeSelection";
        type Type = super::NoiseCancelingModeSelection;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for NoiseCancelingModeSelection {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            [
                (NoiseCancelingMode::Indoor, &self.indoor_mode.get()),
                (NoiseCancelingMode::Outdoor, &self.outdoor_mode.get()),
                (NoiseCancelingMode::Transport, &self.transport_mode.get()),
                (NoiseCancelingMode::Custom, &self.custom_mode.get()),
            ]
            .into_iter()
            .for_each(|(button_noise_canceling_mode, button)| {
                obj.bind_property("noise_canceling_mode", button, "active")
                    .transform_to(
                        move |_, selected_noise_canceling_mode: GlibNoiseCancelingModeValue| {
                            Some(button_noise_canceling_mode == selected_noise_canceling_mode.0)
                        },
                    )
                    .transform_from(move |_, is_active| {
                        if is_active {
                            Some(GlibNoiseCancelingModeValue(button_noise_canceling_mode))
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

    impl WidgetImpl for NoiseCancelingModeSelection {}
    impl BoxImpl for NoiseCancelingModeSelection {}
}

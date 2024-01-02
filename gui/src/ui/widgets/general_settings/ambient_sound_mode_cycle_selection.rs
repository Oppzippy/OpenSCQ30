use gtk::glib::{self, subclass::types::ObjectSubclassIsExt, Object};
use openscq30_lib::devices::standard::structures::AmbientSoundModeCycle;

glib::wrapper! {
    pub struct AmbientSoundModeCycleSelection(ObjectSubclass<imp::AmbientSoundModeCycleSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl AmbientSoundModeCycleSelection {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_ambient_sound_mode_cycle(&self, cycle: &AmbientSoundModeCycle) {
        self.imp().set_ambient_sound_mode_cycle(cycle)
    }

    pub fn ambient_sound_mode_cycle(&self) -> AmbientSoundModeCycle {
        self.imp().ambient_sound_mode_cycle()
    }
}

mod imp {
    // Properties macro creates an enum for internal use. We don't care that it is caught by this lint.
    #![allow(clippy::enum_variant_names)]

    use std::cell::Cell;

    use gtk::{
        glib::{self, clone, subclass::Signal, ParamSpec, Properties, Value},
        prelude::*,
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        CompositeTemplate, TemplateChild,
    };
    use once_cell::sync::Lazy;
    use openscq30_lib::devices::standard::structures::AmbientSoundModeCycle;

    use crate::objects::GlibAmbientSoundModeCycleValue;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/ambient_sound_mode_cycle_selection.ui"
    )]
    #[properties(wrapper_type=super::AmbientSoundModeCycleSelection)]
    pub struct AmbientSoundModeCycleSelection {
        #[template_child]
        pub ambient_sound_mode_cycle_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub normal_mode: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub transparency_mode: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub noise_canceling_mode: TemplateChild<gtk::ToggleButton>,

        #[property(set, get)]
        has_noise_canceling_mode: Cell<bool>,

        ignore_events: Cell<bool>,
    }

    impl AmbientSoundModeCycleSelection {
        pub fn ambient_sound_mode_cycle(&self) -> AmbientSoundModeCycle {
            AmbientSoundModeCycle {
                noise_canceling_mode: self.noise_canceling_mode.is_active(),
                transparency_mode: self.transparency_mode.is_active(),
                normal_mode: self.normal_mode.is_active(),
            }
        }
        pub fn set_ambient_sound_mode_cycle(&self, cycle: &AmbientSoundModeCycle) {
            self.ignore_events.set(true);

            self.normal_mode.set_active(cycle.normal_mode);
            self.transparency_mode.set_active(cycle.transparency_mode);
            self.noise_canceling_mode
                .set_active(cycle.noise_canceling_mode);

            self.ignore_events.set(false);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AmbientSoundModeCycleSelection {
        const NAME: &'static str = "OpenSCQ30AmbientSoundModeCycleSelection";
        type Type = super::AmbientSoundModeCycleSelection;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AmbientSoundModeCycleSelection {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj()
                .bind_property(
                    "has-noise-canceling-mode",
                    &self.noise_canceling_mode.get(),
                    "visible",
                )
                .sync_create()
                .build();

            // TODO switch to a property instead of a signal
            [
                &self.noise_canceling_mode.get(),
                &self.transparency_mode.get(),
                &self.normal_mode.get(),
            ]
            .into_iter()
            .for_each(|button| {
                button.connect_notify_local(
                    Some("active"),
                    clone!(@weak self as this => move |_, _| {
                        if !this.ignore_events.get() {
                            this.obj().emit_by_name::<()>(
                                "ambient-sound-mode-cycle-changed",
                                &[&GlibAmbientSoundModeCycleValue(this.ambient_sound_mode_cycle())],
                            );
                        }
                    }),
                );
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

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("ambient-sound-mode-cycle-changed")
                    .param_types([GlibAmbientSoundModeCycleValue::static_type()])
                    .build()]
            });
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for AmbientSoundModeCycleSelection {}
    impl BoxImpl for AmbientSoundModeCycleSelection {}
}

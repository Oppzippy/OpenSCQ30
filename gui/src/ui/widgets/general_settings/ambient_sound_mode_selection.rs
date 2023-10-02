use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::packets::structures::DeviceFeatureFlags;

glib::wrapper! {
    pub struct AmbientSoundModeSelection(ObjectSubclass<imp::AmbientSoundModeSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl AmbientSoundModeSelection {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_device_feature_flags(&self, flags: &DeviceFeatureFlags) {
        self.imp().set_device_feature_flags(flags);
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
            widget::{
                CompositeTemplateCallbacksClass, CompositeTemplateClass,
                CompositeTemplateInitializingExt, WidgetClassSubclassExt, WidgetImpl,
            },
        },
        CompositeTemplate, TemplateChild,
    };
    use openscq30_lib::packets::structures::{AmbientSoundMode, DeviceFeatureFlags};

    use crate::objects::BoxedAmbientSoundMode;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/ambient_sound_mode_selection.ui"
    )]
    #[properties(wrapper_type=super::AmbientSoundModeSelection)]
    pub struct AmbientSoundModeSelection {
        // Ambient Sound Mode
        #[template_child]
        pub ambient_sound_mode_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub normal_mode: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub transparency_mode: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub noise_canceling_mode: TemplateChild<gtk::ToggleButton>,

        #[property(set, get)]
        has_noise_canceling_mode: Cell<bool>,
        #[property(set, get)]
        ambient_sound_mode: Cell<BoxedAmbientSoundMode>,
    }

    #[gtk::template_callbacks]
    impl AmbientSoundModeSelection {
        pub fn set_device_feature_flags(&self, flags: &DeviceFeatureFlags) {
            let obj = self.obj();
            obj.set_has_noise_canceling_mode(
                flags.contains(DeviceFeatureFlags::NOISE_CANCELING_MODE),
            );
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AmbientSoundModeSelection {
        const NAME: &'static str = "OpenSCQ30AmbientSoundModeSelection";
        type Type = super::AmbientSoundModeSelection;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AmbientSoundModeSelection {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.bind_property(
                "has_noise_canceling_mode",
                &self.noise_canceling_mode.get(),
                "visible",
            )
            .sync_create()
            .build();

            [
                (AmbientSoundMode::Normal, &self.normal_mode.get()),
                (
                    AmbientSoundMode::Transparency,
                    &self.transparency_mode.get(),
                ),
                (
                    AmbientSoundMode::NoiseCanceling,
                    &self.noise_canceling_mode.get(),
                ),
            ]
            .into_iter()
            .for_each(|(ambient_sound_mode, button)| {
                obj.bind_property("ambient_sound_mode", button, "active")
                    .transform_to(
                        move |_, selected_ambient_sound_mode: BoxedAmbientSoundMode| {
                            Some(ambient_sound_mode == selected_ambient_sound_mode.0)
                        },
                    )
                    .transform_from(move |_, is_active| {
                        if is_active {
                            Some(BoxedAmbientSoundMode(ambient_sound_mode))
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

    impl WidgetImpl for AmbientSoundModeSelection {}
    impl BoxImpl for AmbientSoundModeSelection {}
}

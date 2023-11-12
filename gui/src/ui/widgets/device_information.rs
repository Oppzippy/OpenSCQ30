use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::devices::standard::state::DeviceState;

glib::wrapper! {
    pub struct DeviceInformation(ObjectSubclass<imp::DeviceInformation>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DeviceInformation {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_device_state(&self, state: &DeviceState) {
        self.imp().set_device_state(state);
    }
}

mod imp {
    use gtk::{
        glib,
        prelude::ObjectExt,
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, CompositeTemplate,
    };
    use openscq30_lib::devices::standard::state::DeviceState;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/device_information.ui")]
    pub struct DeviceInformation {
        #[template_child]
        firmware_version_label: TemplateChild<gtk::Label>,
        #[template_child]
        firmware_version: TemplateChild<gtk::Label>,

        #[template_child]
        serial_number_label: TemplateChild<gtk::Label>,
        #[template_child]
        serial_number: TemplateChild<gtk::Label>,

        #[template_child]
        age_range_label: TemplateChild<gtk::Label>,
        #[template_child]
        age_range: TemplateChild<gtk::Label>,

        #[template_child]
        feature_flags_label: TemplateChild<gtk::Label>,
        #[template_child]
        feature_flags: TemplateChild<gtk::Label>,
    }

    #[template_callbacks]
    impl DeviceInformation {
        pub fn set_device_state(&self, state: &DeviceState) {
            self.serial_number
                .set_text(state.serial_number.to_owned().unwrap_or_default().as_str());
            self.firmware_version.set_text(
                &state
                    .firmware_version
                    .map(|version| version.to_string())
                    .unwrap_or_default(),
            );
            self.age_range.set_text(
                &state
                    .age_range
                    .map(|age_range| age_range.0.to_string())
                    .unwrap_or_default(),
            );
            // TODO display as JSON or something
            self.feature_flags
                .set_text(&format!("{:?}", state.device_profile));
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DeviceInformation {
        const NAME: &'static str = "OpenSCQ30DeviceInformation";
        type Type = super::DeviceInformation;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for DeviceInformation {
        fn constructed(&self) {
            [
                (
                    &self.firmware_version_label.get(),
                    &self.firmware_version.get(),
                ),
                (&self.serial_number_label.get(), &self.serial_number.get()),
                (&self.age_range_label.get(), &self.age_range.get()),
                (&self.feature_flags_label.get(), &self.feature_flags.get()),
            ]
            .into_iter()
            .for_each(|(label, value)| {
                // value hides itself when empty
                value
                    .bind_property("label", value, "visible")
                    .transform_to(|_, text: &str| Some(!text.is_empty()))
                    .sync_create()
                    .build();
                // value hides the label when empty
                value
                    .bind_property("label", label, "visible")
                    .transform_to(|_, text: &str| Some(!text.is_empty()))
                    .sync_create()
                    .build();
            })
        }
    }
    impl WidgetImpl for DeviceInformation {}
    impl BoxImpl for DeviceInformation {}
}

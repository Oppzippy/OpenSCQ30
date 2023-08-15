use gtk::{
    glib,
    prelude::ObjectExt,
    subclass::{
        prelude::*,
        widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
    },
    template_callbacks, CompositeTemplate,
};
use itertools::Itertools;
use openscq30_lib::state::DeviceState;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/OpenSCQ30/device_information/template.ui")]
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
        self.firmware_version.set_text(&match (
            state.left_firmware_version,
            state.right_firmware_version,
        ) {
            (Some(left), Some(right)) => {
                format!("{}, {}", left.to_string(), right.to_string())
            }
            (Some(left), None) => left.to_string(),
            _ => "".to_owned(),
        });
        self.age_range.set_text(
            &state
                .age_range
                .map(|age_range| age_range.0.to_string())
                .unwrap_or_default(),
        );
        self.feature_flags
            .set_text(&state.feature_flags.iter_names().map(|x| x.0).join("\n"));
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

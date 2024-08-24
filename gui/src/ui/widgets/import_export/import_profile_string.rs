use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::objects::GlibCustomEqualizerProfile;

glib::wrapper! {
    pub struct ImportProfileString(ObjectSubclass<imp::ImportProfileString>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImportProfileString {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn profiles_or_show_parse_error(&self) -> Option<Vec<GlibCustomEqualizerProfile>> {
        self.imp().profiles_or_show_parse_error()
    }
}

mod imp {
    use std::sync::OnceLock;

    use gtk::{
        glib::{self, subclass::Signal},
        prelude::*,
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, CompositeTemplate,
    };

    use crate::{
        objects::GlibCustomEqualizerProfile,
        ui::widgets::import_export::serialization::IOCustomEqualizerProfile,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/import_export/import_profile_string.ui"
    )]
    pub struct ImportProfileString {
        #[template_child]
        entry: TemplateChild<gtk::Entry>,
        #[template_child]
        error_text: TemplateChild<gtk::Label>,
    }

    #[template_callbacks]
    impl ImportProfileString {
        pub fn reset(&self) {
            self.entry.buffer().set_text("");
            self.error_text.set_text("");
        }

        pub fn profiles_or_show_parse_error(&self) -> Option<Vec<GlibCustomEqualizerProfile>> {
            match serde_json::from_str::<Vec<IOCustomEqualizerProfile>>(self.entry.text().as_str())
            {
                Ok(profiles) => Some(profiles.into_iter().map(|profile| profile.into()).collect()),
                Err(err) => {
                    self.error_text.set_text(&format!("{err}"));
                    None
                }
            }
        }

        pub fn text(&self) -> String {
            self.entry.text().into()
        }

        #[template_callback]
        fn handle_next_clicked(&self, _: gtk::Button) {
            self.obj().emit_by_name::<()>("next", &[]);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImportProfileString {
        const NAME: &'static str = "OpenSCQ30ImportProfileString";
        type Type = super::ImportProfileString;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImportProfileString {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("next").build()])
        }
    }
    impl WidgetImpl for ImportProfileString {}
    impl BoxImpl for ImportProfileString {}
}

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct ImportExportMenu(ObjectSubclass<imp::ImportExportMenu>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImportExportMenu {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use std::sync::OnceLock;

    use gtk::{
        glib::{self, subclass::Signal},
        prelude::ObjectExt,
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, CompositeTemplate,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/import_export/import_export_menu.ui")]
    pub struct ImportExportMenu {}

    #[template_callbacks]
    impl ImportExportMenu {
        #[template_callback]
        fn handle_export_equalizer_profiles_clicked(&self, _: adw::ActionRow) {
            self.obj().emit_by_name("export-equalizer-profiles", &[])
        }

        #[template_callback]
        fn handle_import_equalizer_profiles_clicked(&self, _: adw::ActionRow) {
            self.obj().emit_by_name("import-equalizer-profiles", &[])
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImportExportMenu {
        const NAME: &'static str = "OpenSCQ30ImportExportMenu";
        type Type = super::ImportExportMenu;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImportExportMenu {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("export-equalizer-profiles").build(),
                    Signal::builder("import-equalizer-profiles").build(),
                ]
            })
        }
    }
    impl WidgetImpl for ImportExportMenu {}
    impl BoxImpl for ImportExportMenu {}
}

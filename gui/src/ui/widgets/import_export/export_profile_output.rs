use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct ExportProfileOutput(ObjectSubclass<imp::ExportProfileOutput>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ExportProfileOutput {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use std::{cell::RefCell, sync::OnceLock};

    use gtk::{
        glib::{self, subclass::Signal},
        prelude::*,
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, CompositeTemplate,
    };

    #[derive(Default, CompositeTemplate)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/import_export/export_profile_output.ui"
    )]
    pub struct ExportProfileOutput {
        #[template_child]
        text_view: TemplateChild<gtk::TextView>,

        text: RefCell<String>,
    }

    #[template_callbacks]
    impl ExportProfileOutput {
        pub fn reset(&self) {
            self.set_text("");
        }

        pub fn set_text(&self, text: &str) {
            self.text_view.buffer().set_text(text);
            *self.text.borrow_mut() = text.to_string();
        }

        #[template_callback]
        fn handle_copy_clicked(&self, _: gtk::Button) {
            self.obj().clipboard().set_text(&self.text.borrow());
        }

        #[template_callback]
        fn handle_done_clicked(&self, _: gtk::Button) {
            self.obj().emit_by_name("done", &[])
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExportProfileOutput {
        const NAME: &'static str = "OpenSCQ30ExportProfileOutput";
        type Type = super::ExportProfileOutput;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExportProfileOutput {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("done").build()])
        }
    }
    impl WidgetImpl for ExportProfileOutput {}
    impl BoxImpl for ExportProfileOutput {}
}

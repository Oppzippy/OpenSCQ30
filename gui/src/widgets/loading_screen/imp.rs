use gtk::{
    glib::{self, subclass::Signal},
    prelude::ObjectExt,
    subclass::{
        prelude::*,
        widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
    },
    template_callbacks, CompositeTemplate,
};
use once_cell::sync::Lazy;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/OpenSCQ30/loading_screen/template.ui")]
pub struct LoadingScreen {}

#[template_callbacks]
impl LoadingScreen {
    #[template_callback]
    fn handle_cancel_clicked(&self, _: gtk::Button) {
        self.obj().emit_by_name("cancel", &[])
    }
}

#[glib::object_subclass]
impl ObjectSubclass for LoadingScreen {
    const NAME: &'static str = "OpenSCQ30LoadingScreen";
    type Type = super::LoadingScreen;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for LoadingScreen {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("cancel").build()]);
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for LoadingScreen {}
impl BoxImpl for LoadingScreen {}

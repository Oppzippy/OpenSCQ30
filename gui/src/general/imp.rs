use gtk::{
    glib,
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass},
        widget::{CompositeTemplateClass, WidgetImpl},
    },
    CompositeTemplate,
};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/general.ui")]
pub struct General {}

impl General {}

#[glib::object_subclass]
impl ObjectSubclass for General {
    const NAME: &'static str = "OpenSCQ30General";
    type Type = super::General;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for General {}
impl WidgetImpl for General {}
impl BoxImpl for General {}

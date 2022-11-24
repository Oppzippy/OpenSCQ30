use gtk::{
    glib,
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass},
        widget::{CompositeTemplateClass, WidgetImpl},
    },
    CompositeTemplate, TemplateChild,
};

use crate::{equalizer::Equalizer, general::General};
use gtk::subclass::widget::WidgetClassSubclassExt;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/main_window.ui")]
pub struct MainWindow {
    #[template_child]
    pub general: TemplateChild<General>,
    #[template_child]
    pub equalizer: TemplateChild<Equalizer>,
}

impl MainWindow {}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "OpenSCQ30MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {}
impl WidgetImpl for MainWindow {}
impl BoxImpl for MainWindow {}

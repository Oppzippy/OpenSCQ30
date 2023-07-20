use gtk::{
    glib::{self, Sender},
    subclass::{
        prelude::*,
        widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
    },
    template_callbacks, CompositeTemplate,
};
use once_cell::sync::OnceCell;

use crate::actions::Action;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/OpenSCQ30/loading_screen/template.ui")]
pub struct LoadingScreen {
    sender: OnceCell<Sender<Action>>,
}

#[template_callbacks]
impl LoadingScreen {
    pub fn set_sender(&self, sender: Sender<Action>) {
        self.sender.set(sender).unwrap();
    }

    #[template_callback]
    fn handle_cancel_clicked(&self, _: gtk::Button) {
        self.sender.get().unwrap().send(Action::Disconnect).unwrap();
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

impl ObjectImpl for LoadingScreen {}
impl WidgetImpl for LoadingScreen {}
impl BoxImpl for LoadingScreen {}

mod imp;

use gtk::{
    glib::{self, Object, Sender},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::actions::Action;

glib::wrapper! {
    pub struct LoadingScreen(ObjectSubclass<imp::LoadingScreen>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl LoadingScreen {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: Sender<Action>) {
        self.imp().set_sender(sender);
    }
}

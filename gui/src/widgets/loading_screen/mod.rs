mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct LoadingScreen(ObjectSubclass<imp::LoadingScreen>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl LoadingScreen {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

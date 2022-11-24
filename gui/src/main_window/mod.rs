mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl MainWindow {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

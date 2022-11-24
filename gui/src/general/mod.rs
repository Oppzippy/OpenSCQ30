mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct General(ObjectSubclass<imp::General>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl General {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

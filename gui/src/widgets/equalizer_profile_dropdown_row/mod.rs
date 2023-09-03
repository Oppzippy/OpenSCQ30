mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct EqualizerProfileDropdownRow(ObjectSubclass<imp::EqualizerProfileDropdownRow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EqualizerProfileDropdownRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn volume_adjustments(&self) -> Option<[f64; 8]> {
        self.imp().volume_adjustments.get()
    }

    pub fn set_volume_adjustments(&self, volume_adjustments: Option<[f64; 8]>) {
        self.imp().set_volume_adjustments(volume_adjustments)
    }
}

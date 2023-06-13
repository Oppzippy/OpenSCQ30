mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

glib::wrapper! {
    pub struct Equalizer(ObjectSubclass<imp::Equalizer>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Equalizer {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn volume_adjustments(&self) -> [i8; 8] {
        return self.imp().volume_adjustments();
    }

    pub fn set_volumes(&self, volumes: [i8; 8]) {
        self.imp().set_volumes(volumes);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use gtk::{
        glib::{self, closure_local},
        prelude::ObjectExt,
        subclass::prelude::ObjectSubclassIsExt,
    };

    use crate::load_resources;

    use super::Equalizer;

    #[gtk::test]
    fn test_set_and_get_volumes() {
        load_resources();
        let equalizer = Equalizer::new();
        let expected_volumes = [0, 1, 2, 3, 4, 5, 6, 7];
        equalizer.set_volumes(expected_volumes.to_owned());
        assert_eq!(equalizer.volume_adjustments(), expected_volumes);
    }

    #[gtk::test]
    async fn test_volume_changed_signal() {
        load_resources();
        let equalizer = Equalizer::new();
        let received_event = Rc::new(Cell::new(false));
        equalizer.connect_closure(
            "volumes-changed",
            false,
            closure_local!(@strong received_event => move |_: Equalizer| {
                received_event.set(true);
            }),
        );
        equalizer.imp().band_100.set_volume(1);
        assert_eq!(received_event.get(), true);
    }

    #[gtk::test]
    async fn test_set_volumes_does_not_fire_signal() {
        load_resources();
        let equalizer = Equalizer::new();
        let received_event = Rc::new(Cell::new(false));
        equalizer.connect_closure(
            "volumes-changed",
            false,
            closure_local!(@strong received_event => move |_: Equalizer| {
                received_event.set(true);
            }),
        );
        equalizer.set_volumes([0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(received_event.get(), false);
    }
}

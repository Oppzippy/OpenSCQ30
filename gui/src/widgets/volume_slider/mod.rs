mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct VolumeSlider(ObjectSubclass<imp::VolumeSlider>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl VolumeSlider {
    pub fn new(band: i32, volume: f64) -> Self {
        Object::builder()
            .property("band", &band)
            .property("volume", &volume)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use gtk::subclass::prelude::*;

    use crate::load_resources;

    use super::VolumeSlider;

    #[gtk::test]
    fn test_hz_below_1k() {
        load_resources();
        let slider = VolumeSlider::new(999, 0.0);
        let text = slider.imp().band_label.text();
        assert_eq!(text, "999 Hz");
    }

    #[gtk::test]
    fn test_hz_at_1k() {
        load_resources();
        let slider = VolumeSlider::new(1000, 0.0);
        let text = slider.imp().band_label.text();
        assert_eq!(text, "1.0 kHz");
    }

    #[gtk::test]
    fn test_hz_above_1k() {
        load_resources();
        let slider = VolumeSlider::new(1001, 0.0);
        let text = slider.imp().band_label.text();
        assert_eq!(text, "1.0 kHz");
    }
}

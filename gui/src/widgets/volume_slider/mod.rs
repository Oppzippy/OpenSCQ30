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
            .property("band", band)
            .property("volume-slider-value", volume)
            .build()
    }

    pub fn volume(&self) -> f64 {
        self.volume_slider_value()
    }

    pub fn set_volume(&self, volume: f64) {
        self.set_volume_slider_value(volume);
    }
}

#[cfg(test)]
mod tests {
    use gtk::{subclass::prelude::*, traits::*};

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

    #[gtk::test]
    fn test_slider_changes_text() {
        load_resources();
        let slider = VolumeSlider::new(80, 1.0);
        assert_eq!(slider.imp().text_input.text(), "1.0");
        slider.imp().slider.set_value(-1.1);
        assert_eq!(slider.imp().text_input.text(), "-1.1");
    }

    #[gtk::test]
    async fn test_text_changes_slider() {
        load_resources();
        let slider = VolumeSlider::new(80, 2.0);
        assert_eq!(slider.imp().slider.value(), 2.0);
        slider.imp().text_input.set_value(-2.1);
        assert_eq!(slider.imp().slider.value(), -2.1);
    }

    #[gtk::test]
    fn test_set_volume() {
        load_resources();
        let slider = VolumeSlider::new(80, 0.0);
        slider.set_volume(1.5);
        assert_eq!(slider.imp().slider.value(), 1.5);
    }

    #[gtk::test]
    fn test_get_volume() {
        load_resources();
        let slider = VolumeSlider::new(80, 0.0);
        slider.imp().slider.set_value(1.5);
        assert_eq!(slider.volume(), 1.5);
    }
}

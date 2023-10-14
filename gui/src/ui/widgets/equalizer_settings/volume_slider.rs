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

mod imp {
    use std::cell::Cell;

    use gtk::{
        glib::{self, BindingFlags, ParamSpec, Properties, Value},
        prelude::*,
        subclass::{
            prelude::{BoxImpl, DerivedObjectProperties, ObjectImpl, ObjectSubclass, *},
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        traits::{RangeExt, ScaleExt},
        CompositeTemplate, Label, Scale, SpinButton, TemplateChild,
    };
    use openscq30_lib::packets::structures::VolumeAdjustments;

    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::VolumeSlider)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/equalizer_settings/volume_slider.ui")]
    pub struct VolumeSlider {
        #[template_child]
        pub slider: TemplateChild<Scale>,
        #[template_child]
        pub text_input: TemplateChild<SpinButton>,
        #[template_child]
        pub band_label: TemplateChild<Label>,

        #[property(get, set)]
        pub volume_slider_value: Cell<f64>,
        #[property(get, set)]
        pub band: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VolumeSlider {
        const NAME: &'static str = "OpenSCQ30VolumeSlider";
        type Type = super::VolumeSlider;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VolumeSlider {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.bind_property("band", &self.band_label.get(), "label")
                .transform_to(|_, band: i32| {
                    if band >= 1000 {
                        Some(format!("{:.1} kHz", band as f64 / 1000.0))
                    } else {
                        Some(format!("{} Hz", band))
                    }
                })
                .flags(BindingFlags::SYNC_CREATE)
                .build();

            let slider = self.slider.get();
            let lower = VolumeAdjustments::MIN_VOLUME;
            let upper = VolumeAdjustments::MAX_VOLUME;
            slider.adjustment().set_lower(lower);
            slider.adjustment().set_upper(upper);
            slider.add_mark(lower, gtk::PositionType::Right, Some(&format!("{}", lower)));
            slider.add_mark(0.0, gtk::PositionType::Right, Some("0"));
            slider.add_mark(
                upper,
                gtk::PositionType::Right,
                Some(&format!("+{}", upper)),
            );
        }

        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            Self::derived_set_property(self, id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            Self::derived_property(self, id, pspec)
        }
    }
    impl WidgetImpl for VolumeSlider {}
    impl BoxImpl for VolumeSlider {}
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

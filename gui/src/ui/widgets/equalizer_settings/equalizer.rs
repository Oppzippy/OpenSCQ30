use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::devices::standard::structures::VolumeAdjustments;

glib::wrapper! {
    pub struct Equalizer(ObjectSubclass<imp::Equalizer>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Equalizer {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn volume_adjustments(&self) -> VolumeAdjustments {
        return self.imp().volume_adjustments();
    }

    pub fn set_volumes(&self, volumes: &[f64]) {
        self.imp().set_volumes(volumes);
    }
}

mod imp {
    use std::cell::{Cell, RefCell};
    use std::sync::LazyLock;

    use gtk::glib::clone;
    use gtk::{
        glib::{self, subclass::Signal},
        prelude::*,
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        CompositeTemplate, TemplateChild,
    };
    use openscq30_lib::devices::standard::structures::VolumeAdjustments;

    use crate::ui::widgets::equalizer_settings::volume_slider::VolumeSlider;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/oppzippy/OpenSCQ30/ui/widgets/equalizer_settings/equalizer.ui")]
    pub struct Equalizer {
        #[template_child]
        pub band_100: TemplateChild<VolumeSlider>,
        #[template_child]
        pub band_200: TemplateChild<VolumeSlider>,
        #[template_child]
        pub band_400: TemplateChild<VolumeSlider>,
        #[template_child]
        pub band_800: TemplateChild<VolumeSlider>,
        #[template_child]
        pub band_1600: TemplateChild<VolumeSlider>,
        #[template_child]
        pub band_3200: TemplateChild<VolumeSlider>,
        #[template_child]
        pub band_6400: TemplateChild<VolumeSlider>,
        #[template_child]
        pub band_12800: TemplateChild<VolumeSlider>,

        dont_fire_events: Cell<bool>,
        bands_past_8: RefCell<Vec<f64>>,
    }

    impl Equalizer {
        pub fn volume_adjustments(&self) -> VolumeAdjustments {
            VolumeAdjustments::new(
                self.get_volume_sliders()
                    .iter()
                    .map(|slider| slider.volume())
                    .chain(self.bands_past_8.borrow().iter().cloned()),
            )
            .expect("we should not allow displaying an invalid number of bands")
        }

        pub fn set_volumes(&self, volumes: &[f64]) {
            // TODO set number of visible sliders once a number other than 8 is needed by a device
            {
                let mut bands_past_8 = self.bands_past_8.borrow_mut();
                bands_past_8.clear();
                bands_past_8.extend(volumes[8..].iter());
            }

            self.dont_fire_events.set(true);
            self.get_volume_sliders()
                .iter()
                .zip(volumes)
                .for_each(|(slider, volume)| slider.set_volume(*volume));
            self.dont_fire_events.set(false);
        }

        fn handle_volume_change(&self) {
            if !self.dont_fire_events.get() {
                self.obj().emit_by_name::<()>("volumes-changed", &[]);
            }
        }

        fn get_volume_sliders(&self) -> Vec<&TemplateChild<VolumeSlider>> {
            [
                &self.band_100,
                &self.band_200,
                &self.band_400,
                &self.band_800,
                &self.band_1600,
                &self.band_3200,
                &self.band_6400,
                &self.band_12800,
            ]
            .to_vec()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Equalizer {
        const NAME: &'static str = "OpenSCQ30Equalizer";
        type Type = super::Equalizer;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Equalizer {
        fn constructed(&self) {
            for band in self.get_volume_sliders() {
                band.connect_notify_local(
                    Some("volume-slider-value"),
                    clone!(@weak self as this => move |_slider, _param_spec| {
                        this.handle_volume_change();
                    }),
                );
            }
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: LazyLock<Vec<Signal>> =
                LazyLock::new(|| vec![Signal::builder("volumes-changed").build()]);
            SIGNALS.as_ref()
        }
    }
    impl WidgetImpl for Equalizer {}
    impl BoxImpl for Equalizer {}
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
        let expected_volumes = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7];
        equalizer.set_volumes(&expected_volumes.to_owned());
        assert_eq!(
            expected_volumes,
            equalizer.volume_adjustments().adjustments().as_ref()
        );
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
        equalizer.imp().band_100.set_volume(0.1);
        assert_eq!(true, received_event.get());
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
        equalizer.set_volumes(&[0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]);
        assert_eq!(false, received_event.get());
    }

    #[gtk::test]
    async fn test_input_number_of_bands_matches_output_number_of_bands() {
        load_resources();
        let equalizer = Equalizer::new();
        let volumes: [f64; 10] = Default::default();
        equalizer.set_volumes(&volumes);
        assert_eq!(
            volumes.len(),
            equalizer.volume_adjustments().adjustments().len()
        );
    }

    #[gtk::test]
    async fn test_bands_past_8_keep_their_values() {
        load_resources();
        let equalizer = Equalizer::new();
        let volumes: [f64; 10] = std::array::from_fn(|i| i as f64);
        equalizer.set_volumes(&volumes);

        let volume_adjustments = equalizer.volume_adjustments().adjustments();
        assert_eq!(8.0, volume_adjustments[8]);
        assert_eq!(9.0, volume_adjustments[9]);
    }
}

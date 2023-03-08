use gtk::glib::clone;
use gtk::glib::subclass::Signal;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassExt;
use gtk::subclass::widget::{CompositeTemplateInitializingExt, WidgetClassSubclassExt};
use gtk::{
    glib,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass},
        widget::{CompositeTemplateClass, WidgetImpl},
    },
    CompositeTemplate, TemplateChild,
};
use once_cell::sync::Lazy;

use crate::widgets::VolumeSlider;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/OpenSCQ30/equalizer/template.ui")]
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
}

impl Equalizer {
    pub fn volumes(&self) -> [i8; 8] {
        self.get_volume_sliders()
            .map(|slider| slider.volume() as i8)
    }

    pub fn set_volumes(&self, volumes: [i8; 8]) {
        self.get_volume_sliders()
            .iter()
            .zip(volumes.into_iter())
            .for_each(|(slider, volume)| slider.set_volume(volume as f64));
    }

    fn handle_volume_change(&self) {
        self.obj().emit_by_name::<()>("volumes-changed", &[]);
    }

    fn get_volume_sliders(&self) -> [&TemplateChild<VolumeSlider>; 8] {
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
                Some("volume"),
                clone!(@weak self as this => move |_slider, _param_spec| {
                    this.handle_volume_change();
                }),
            );
        }
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> =
            Lazy::new(|| vec![Signal::builder("volumes-changed").build()]);
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for Equalizer {}
impl BoxImpl for Equalizer {}

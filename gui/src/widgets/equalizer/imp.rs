use gtk::subclass::widget::WidgetClassSubclassExt;
use gtk::{
    glib,
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass},
        widget::{CompositeTemplateClass, WidgetImpl},
    },
    CompositeTemplate, TemplateChild,
};

use crate::widgets::VolumeSlider;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/equalizer.ui")]
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
        [
            self.band_100.get().volume(),
            self.band_200.get().volume(),
            self.band_400.get().volume(),
            self.band_800.get().volume(),
            self.band_1600.get().volume(),
            self.band_3200.get().volume(),
            self.band_6400.get().volume(),
            self.band_12800.get().volume(),
        ]
    }

    pub fn set_volumes(&self, volumes: [i8; 8]) {
        self.band_100.get().set_volume(volumes[0]);
        self.band_200.get().set_volume(volumes[1]);
        self.band_400.get().set_volume(volumes[2]);
        self.band_800.get().set_volume(volumes[3]);
        self.band_1600.get().set_volume(volumes[4]);
        self.band_3200.get().set_volume(volumes[5]);
        self.band_6400.get().set_volume(volumes[6]);
        self.band_12800.get().set_volume(volumes[7]);
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

impl ObjectImpl for Equalizer {}
impl WidgetImpl for Equalizer {}
impl BoxImpl for Equalizer {}

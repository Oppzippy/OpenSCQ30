use std::cell::Cell;

use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{BindingFlags, ParamSpec, ParamSpecDouble, ParamSpecInt};
use gtk::prelude::*;
use gtk::subclass::prelude::{ObjectImplExt, ObjectSubclassExt};
use gtk::subclass::widget::{CompositeTemplateInitializingExt, WidgetClassSubclassExt};
use gtk::traits::{RangeExt, ScaleExt};
use gtk::Label;
use gtk::{
    glib,
    subclass::{
        prelude::{BoxImpl, ObjectImpl, ObjectSubclass},
        widget::{CompositeTemplateClass, WidgetImpl},
    },
    CompositeTemplate, Scale, TemplateChild,
};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/oppzippy/openscq30/volume_slider/template.ui")]
pub struct VolumeSlider {
    #[template_child]
    pub slider: TemplateChild<Scale>,
    #[template_child]
    pub band_label: TemplateChild<Label>,

    pub volume: Cell<f64>,
    pub band: Cell<i32>,
}

impl VolumeSlider {
    pub fn volume(&self) -> i8 {
        (self.volume.get() * 10.0).clamp(-60.0, 60.0) as i8
    }

    pub fn set_volume(&self, volume: i8) {
        self.obj().set_property("volume", volume as f64 / 10.0);
    }
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
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecInt::builder("band").build(),
                ParamSpecDouble::builder("volume").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "band" => {
                let band = value.get().expect("band must be i32");
                self.band.replace(band);
            }
            "volume" => {
                let volume = value.get().expect("volume must be f64");
                self.volume.replace(volume);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, _pspec: &ParamSpec) -> glib::Value {
        match _pspec.name() {
            "band" => self.band.get().to_value(),
            "volume" => self.volume.get().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.bind_property("band", &self.band_label.get(), "label")
            .transform_to(|_, band: i32| {
                if band > 1000 {
                    Some(format!("{:.1} kHz", band as f64 / 1000.0))
                } else {
                    Some(format!("{} Hz", band))
                }
            })
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        obj.bind_property("volume", &self.slider.get().adjustment(), "value")
            .flags(BindingFlags::BIDIRECTIONAL)
            .build();

        let slider = self.slider.get();

        slider.add_mark(-6.0, gtk::PositionType::Right, Some("-6"));
        slider.add_mark(0.0, gtk::PositionType::Right, Some("0"));
        slider.add_mark(6.0, gtk::PositionType::Right, Some("+6"));
    }
}
impl WidgetImpl for VolumeSlider {}
impl BoxImpl for VolumeSlider {}

use std::cell::Cell;

use gtk::glib::{BindingFlags, ParamSpec, Properties, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::{ObjectImplExt, ObjectSubclassExt};
use gtk::subclass::widget::{CompositeTemplateInitializingExt, WidgetClassSubclassExt};
use gtk::traits::{RangeExt, ScaleExt};
use gtk::Label;
use gtk::{
    glib,
    subclass::{
        prelude::{BoxImpl, DerivedObjectProperties, ObjectImpl, ObjectSubclass},
        widget::{CompositeTemplateClass, WidgetImpl},
    },
    CompositeTemplate, Scale, TemplateChild,
};

#[derive(Default, CompositeTemplate, Properties)]
#[properties(wrapper_type = super::VolumeSlider)]
#[template(resource = "/com/oppzippy/OpenSCQ30/volume_slider/template.ui")]
pub struct VolumeSlider {
    #[template_child]
    pub slider: TemplateChild<Scale>,
    #[template_child]
    pub band_label: TemplateChild<Label>,

    #[property(get, set)]
    pub volume: Cell<f64>,
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
                    Some(format!("{:.1}", band as f64 / 1000.0))
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
        slider.set_format_value_func(|_slider, value| format!("{:.1}", value / 10.0));

        slider.add_mark(-60.0, gtk::PositionType::Right, Some("-6"));
        slider.add_mark(0.0, gtk::PositionType::Right, Some("0"));
        slider.add_mark(60.0, gtk::PositionType::Right, Some("+6"));
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

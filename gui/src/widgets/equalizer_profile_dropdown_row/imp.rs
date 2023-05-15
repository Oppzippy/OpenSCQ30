use std::cell::{Cell, RefCell};

use gtk::{
    cairo::{LineCap, LineJoin},
    glib::{self, Properties},
    prelude::*,
    subclass::{
        prelude::*,
        widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
    },
    CompositeTemplate,
};

#[derive(Default, CompositeTemplate, Properties)]
#[properties(wrapper_type = super::EqualizerProfileDropdownRow)]
#[template(resource = "/com/oppzippy/OpenSCQ30/equalizer_profile_dropdown_row/template.ui")]
pub struct EqualizerProfileDropdownRow {
    #[template_child]
    pub label: TemplateChild<gtk::Label>,
    #[template_child]
    pub drawing_area: TemplateChild<gtk::DrawingArea>,

    #[property(get, set)]
    pub name: RefCell<String>,
    pub volume_adjustments: Cell<Option<[i8; 8]>>,
}

impl EqualizerProfileDropdownRow {
    pub fn set_volume_adjustments(&self, volume_adjustments: Option<[i8; 8]>) {
        self.volume_adjustments.set(volume_adjustments);
        self.drawing_area.queue_draw();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for EqualizerProfileDropdownRow {
    const NAME: &'static str = "OpenSCQ30EqualizerProfileDropdownRow";
    type Type = super::EqualizerProfileDropdownRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for EqualizerProfileDropdownRow {
    fn constructed(&self) {
        self.obj()
            .bind_property("name", &self.label.get(), "label")
            .build();
        self.drawing_area.set_content_width(80);
        let this = self.to_owned();
        self.label.connect_label_notify(move |label| {
            this.drawing_area.set_content_height(label.height());
        });
        let this = self.to_owned();
        self.drawing_area
            .set_draw_func(move |drawing_area, context, width, height| {
                if let Some(volume_adjustments) = this.volume_adjustments.get() {
                    let color = drawing_area.style_context().color();
                    context.set_source_rgba(
                        color.red().into(),
                        color.green().into(),
                        color.blue().into(),
                        0.4,
                    );
                    context.set_line_width(2.0);
                    context.set_line_cap(LineCap::Round);
                    context.set_line_join(LineJoin::Round);

                    let padding = 2.0;
                    let width_without_padding = width as f64 - padding * 2.0;
                    let height_without_padding = height as f64 - padding * 2.0;
                    let get_x = |index: usize| {
                        (index as f64 / volume_adjustments.len() as f64) * width_without_padding
                            + padding
                    };
                    let get_y = |value: i8| {
                        (1.0 - (value as f64 + 120.0) / 240.0) * height_without_padding + padding
                    };
                    volume_adjustments
                        .windows(2)
                        .enumerate()
                        .for_each(|(index, adjustment)| {
                            let from_x = get_x(index);
                            let to_x = get_x(index + 1);

                            let from_y = get_y(adjustment[0]);
                            let to_y = get_y(adjustment[1]);

                            context.move_to(from_x, from_y);
                            context.line_to(to_x, to_y);
                        });
                    context.stroke().unwrap();
                }
            });
    }
}
impl WidgetImpl for EqualizerProfileDropdownRow {}
impl BoxImpl for EqualizerProfileDropdownRow {}

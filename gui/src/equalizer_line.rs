use std::marker::PhantomData;

use cosmic::{
    Element,
    cosmic_theme::palette::WithAlpha,
    iced::{Length, Point},
    widget::{self, canvas},
};
use itertools::Itertools;

pub struct EqualizerLine<Message> {
    _message: PhantomData<Message>,
    cache: canvas::Cache,
    min: i16,
    max: i16,
    values: Vec<i16>,
}

impl<Message> EqualizerLine<Message> {
    pub fn new(min: i16, max: i16, values: Vec<i16>) -> Self {
        Self {
            _message: PhantomData,
            cache: Default::default(),
            min,
            max,
            values,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        widget::canvas(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn points(&self, width: f32, height: f32, padding: f32) -> impl Iterator<Item = Point<f32>> {
        let width_without_padding = width - padding * 2.0;
        let height_without_padding = height - padding * 2.0;
        let range = self.max - self.min;
        self.values.iter().enumerate().map(move |(i, v)| {
            let normalized_x = i as f32 / (self.values.len() - 1) as f32;
            let x = normalized_x * width_without_padding + padding;
            let normalized_y = 1f32 - ((v - self.min) as f32 / range as f32);
            let y = normalized_y * height_without_padding + padding;
            Point::new(x, y)
        })
    }
}

impl<Message> canvas::Program<Message, cosmic::Theme, cosmic::Renderer> for EqualizerLine<Message> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &cosmic::Renderer,
        theme: &cosmic::Theme,
        bounds: cosmic::iced::Rectangle,
        _cursor: cosmic::iced_core::mouse::Cursor,
    ) -> Vec<canvas::Geometry<cosmic::iced::Renderer>> {
        let equalizer_line = self.cache.draw(renderer, bounds.size(), |frame| {
            let theme = theme.cosmic();

            for height in [2.0, frame.height() / 2.0, frame.height() - 2.0] {
                frame.stroke(
                    &canvas::Path::line(Point::new(0.0, height), Point::new(frame.width(), height)),
                    canvas::Stroke {
                        style: canvas::stroke::Style::Solid(
                            theme.on_bg_color().with_alpha(0.3).into(),
                        ),
                        width: 1.0,
                        line_cap: canvas::LineCap::Square,
                        line_join: canvas::LineJoin::Bevel,
                        line_dash: canvas::LineDash::default(),
                    },
                );
            }

            let paths = self
                .points(frame.width(), frame.height(), 4f32)
                .tuple_windows()
                .map(|(left, right)| canvas::Path::line(left, right));

            for path in paths {
                frame.stroke(
                    &path,
                    canvas::Stroke {
                        style: canvas::stroke::Style::Solid(theme.on_bg_color().into()),
                        width: 3f32,
                        line_cap: canvas::LineCap::Round,
                        line_join: canvas::LineJoin::Round,
                        line_dash: canvas::LineDash::default(),
                    },
                );
            }
        });
        vec![equalizer_line]
    }
}

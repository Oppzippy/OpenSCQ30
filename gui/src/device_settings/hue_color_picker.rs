use cosmic::{
    Element,
    cosmic_theme::palette::{Hsv, IntoColor, Srgb},
    iced::{Color, alignment},
    widget::{self, canvas},
};
use openscq30_i18n::Translate;
use openscq30_lib::settings::SettingId;

use crate::{device_settings::labeled_setting_row, fl};

pub fn hue_color_picker<'a, M>(
    setting_id: SettingId,
    hue: f32,
    on_change: impl Fn(f32) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    // TODO show a gradient in the background behind the slider
    // not done yet since iced's gradients only support rgb and not hsv, so something custom would
    // be necessary.
    labeled_setting_row(
        setting_id.translate(),
        widget::row![
            widget::canvas(HueBox(hue)).width(24).height(24),
            widget::slider(0.0..=360.0, hue, on_change).description(fl!("color-hue-in-degrees")),
        ]
        .align_y(alignment::Vertical::Center)
        .spacing(12),
    )
}

struct HueBox(f32);

impl<Message> canvas::Program<Message, cosmic::Theme, cosmic::Renderer> for HueBox {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &cosmic::Renderer,
        _theme: &cosmic::Theme,
        bounds: cosmic::iced::Rectangle,
        _cursor: cosmic::iced::core::mouse::Cursor,
    ) -> Vec<canvas::Geometry<cosmic::Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let circle = canvas::Path::circle(frame.center(), frame.width() / 2.0);

        let hsv = Hsv::new(self.0, 1.0, 1.0);
        let rgb: Srgb = hsv.into_color();

        frame.fill(&circle, Color::from_rgb(rgb.red, rgb.green, rgb.blue));
        vec![frame.into_geometry()]
    }
}

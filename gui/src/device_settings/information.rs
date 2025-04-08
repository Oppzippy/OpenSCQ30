use std::borrow::Cow;

use cosmic::{
    Element,
    iced::{Length, alignment},
    widget,
};
use openscq30_i18n::Translate;
use openscq30_lib::api::settings::SettingId;

pub fn information<'a, M>(setting_id: SettingId, text: Cow<'a, str>) -> Element<'a, M>
where
    M: Clone + 'a,
{
    widget::row::with_children(vec![
        widget::text::heading(setting_id.translate())
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .into(),
        widget::text(text)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Left)
            .into(),
    ])
    .spacing(15)
    .align_y(alignment::Vertical::Center)
    .into()
}

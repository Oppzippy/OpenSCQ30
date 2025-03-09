use std::borrow::Cow;

use cosmic::{
    Apply, Element,
    iced::{Length, alignment},
    widget,
};
use openscq30_i18n::Translate;
use openscq30_lib::api::settings::SettingId;

pub fn toggle<'a, M>(
    setting_id: SettingId,
    value: bool,
    on_change: impl Fn(bool) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    with_label(
        setting_id.translate(),
        widget::toggler(value)
            .on_toggle(on_change)
            .apply(widget::container)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right),
    )
}

fn with_label<'a, M>(
    label: impl Into<Cow<'a, str>> + 'a,
    element: impl Into<Element<'a, M>>,
) -> Element<'a, M>
where
    M: 'a,
{
    widget::row()
        .align_y(alignment::Vertical::Center)
        .push(widget::text::text(label).width(Length::Fill))
        .push(element.into())
        .into()
}

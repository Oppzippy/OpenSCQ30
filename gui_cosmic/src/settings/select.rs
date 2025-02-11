use cosmic::{
    iced::{alignment, Length},
    widget, Element,
};
use openscq30_lib::api::settings::{Select, SettingId};

pub fn select<'a, M>(
    setting_id: SettingId<'a>,
    setting: &'a Select,
    value: Option<u16>,
    on_change: impl Fn(usize) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    with_label(
        setting_id.0,
        widget::dropdown(&setting.options, value.map(usize::from), on_change)
            .width(Length::FillPortion(1)),
    )
}

fn with_label<'a, M>(label: &'a str, element: impl Into<Element<'a, M>>) -> Element<'a, M>
where
    M: 'a,
{
    widget::row()
        .align_y(alignment::Vertical::Center)
        .push(widget::text::text(label).width(Length::FillPortion(1)))
        .push(element.into())
        .into()
}

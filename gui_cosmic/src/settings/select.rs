use std::borrow::Cow;

use cosmic::{
    Element,
    iced::{Length, alignment},
    widget,
};
use openscq30_lib::api::settings::{Select, SettingId};

pub fn optional_select<'a, M>(
    setting_id: SettingId<'a>,
    setting: &'a Select,
    value: Option<&str>,
    on_change: impl Fn(Option<&str>) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    let selected_index = value
        .map(|value| setting.options.iter().position(|option| option == value))
        .flatten();
    with_label(
        setting_id.0,
        widget::dropdown(&setting.options, selected_index, move |index| {
            on_change(Some(&setting.options[index]))
        })
        .width(Length::FillPortion(1)),
    )
}

fn with_label<'a, M>(label: Cow<'a, str>, element: impl Into<Element<'a, M>>) -> Element<'a, M>
where
    M: 'a,
{
    widget::row()
        .align_y(alignment::Vertical::Center)
        .push(widget::text::text(label).width(Length::FillPortion(1)))
        .push(element.into())
        .into()
}

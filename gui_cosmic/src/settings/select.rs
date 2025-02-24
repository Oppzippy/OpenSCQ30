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
    on_add: Option<M>,
    on_remove: Option<M>,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    let selected_index = value
        .map(|value| setting.options.iter().position(|option| option == value))
        .flatten();
    let maybe_deselect_message = if value.is_some() {
        Some(on_remove.unwrap_or_else(|| on_change(None)))
    } else {
        None
    };
    with_label(
        setting_id.0,
        widget::row()
            .push(
                widget::dropdown(&setting.options, selected_index, move |index| {
                    on_change(Some(&setting.options[index]))
                })
                .width(Length::FillPortion(1)),
            )
            .push_maybe(maybe_deselect_message.map(|deselect_message| {
                widget::button::icon(widget::icon::from_name("list-remove-symbolic"))
                    .on_press(deselect_message)
            }))
            .push_maybe(on_add.map(|on_add| {
                widget::button::icon(widget::icon::from_name("list-add-symbolic")).on_press(on_add)
            })),
    )
}

pub fn select<'a, M>(
    setting_id: SettingId<'a>,
    setting: &'a Select,
    value: &str,
    on_change: impl Fn(&str) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    let selected_index = setting.options.iter().position(|option| option == value);
    with_label(
        setting_id.0,
        widget::dropdown(&setting.options, selected_index, move |index| {
            on_change(&setting.options[index])
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

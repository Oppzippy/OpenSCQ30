use std::borrow::Cow;

use cosmic::{Element, iced::Length, widget};
use openscq30_i18n::Translate;
use openscq30_lib::settings::{Select, SettingId};

use crate::{device_settings::labeled_setting_row, fl, macros::include_icon};

pub fn select<'a, M>(
    setting_id: SettingId,
    setting: &'a Select,
    value: &str,
    on_change: impl Fn(&str) -> M + Send + Sync + 'static,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    let selected_index = setting.options.iter().position(|option| option == value);
    let options = setting.options.to_owned();
    labeled_setting_row(
        setting_id.translate(),
        widget::dropdown(&setting.localized_options, selected_index, move |index| {
            on_change(&options[index])
        })
        .width(Length::Fill),
    )
}

pub fn optional_select<'a, M>(
    setting_id: SettingId,
    setting: &'a Select,
    value: Option<&str>,
    on_change: impl Fn(Option<&str>) -> M + Send + Sync + 'static,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    let localized_items_with_none_option = [fl!("none")]
        .into_iter()
        .chain(setting.localized_options.to_owned())
        .collect::<Vec<_>>();
    let selected_index = value
        .and_then(|value| {
            setting
                .options
                .iter()
                .position(|option| option == value)
                .map(|pos| pos + 1)
        })
        .unwrap_or_default();
    let options = setting.options.to_owned();
    labeled_setting_row(
        setting_id.translate(),
        widget::row().push(
            widget::dropdown(
                localized_items_with_none_option,
                Some(selected_index),
                move |index| {
                    on_change(if index == 0 {
                        None
                    } else {
                        Some(&options[index - 1])
                    })
                },
            )
            .width(Length::Fill),
        ),
    )
}

pub fn modifiable_select<'a, M>(
    setting_id: SettingId,
    setting: &'a Select,
    value: Option<&str>,
    on_change: impl Fn(&str) -> M + Send + Sync + 'static,
    on_add: M,
    on_remove: M,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    let selected_index =
        value.and_then(|value| setting.options.iter().position(|option| option == value));
    let maybe_deselect_message = value.is_some().then_some(on_remove);
    let options = setting.options.to_owned();
    labeled_setting_row(
        setting_id.translate(),
        widget::row()
            .push(
                widget::dropdown(&setting.localized_options, selected_index, move |index| {
                    on_change(&options[index])
                })
                .width(Length::Fill),
            )
            .push_maybe(maybe_deselect_message.map(|deselect_message| {
                widget::button::icon(include_icon!(
                    "list-remove-symbolic",
                    "../../icons/list-remove-symbolic.svg"
                ))
                .on_press(deselect_message)
            }))
            .push(
                widget::button::icon(include_icon!(
                    "list-add-symbolic",
                    "../../icons/list-add-symbolic.svg"
                ))
                .on_press(on_add),
            ),
    )
}

pub fn multi_select<'a, M>(
    _setting_id: SettingId,
    setting: &'a Select,
    values: &'a [Cow<'static, str>],
    on_change: impl Fn(Vec<Cow<'static, str>>) -> M + Send + Sync + Clone + 'static,
) -> Vec<Element<'a, M>>
where
    M: Clone + 'static,
{
    setting
        .options
        .iter()
        .zip(setting.localized_options.iter())
        .map(|(option, localized_option)| {
            let option = option.clone();
            let values = values.to_vec();
            let on_change = on_change.clone();
            widget::settings::item::builder(localized_option.to_owned())
                .toggler(values.contains(&option), move |is_checked| {
                    if !is_checked {
                        on_change(values.iter().filter(|o| **o != option).cloned().collect())
                    } else {
                        on_change(values.iter().chain([&option]).cloned().collect())
                    }
                })
                .into()
        })
        .collect()
}

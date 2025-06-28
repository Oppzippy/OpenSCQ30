use cosmic::{Element, iced::Length, widget};
use openscq30_i18n::Translate;
use openscq30_lib::api::settings::{Select, SettingId};

use crate::{device_settings::labeled_setting_row, fl};

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
                widget::button::icon(widget::icon::from_name("list-remove-symbolic"))
                    .on_press(deselect_message)
            }))
            .push(
                widget::button::icon(widget::icon::from_name("list-add-symbolic")).on_press(on_add),
            ),
    )
}

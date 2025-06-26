use cosmic::{
    Apply, Element,
    iced::{Length, alignment},
    widget,
};
use openscq30_i18n::Translate;
use openscq30_lib::api::settings::SettingId;

use crate::device_settings::labeled_setting_row;

pub fn toggle<'a, M>(
    setting_id: SettingId,
    value: bool,
    on_change: impl Fn(bool) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    labeled_setting_row(
        setting_id.translate(),
        widget::toggler(value)
            .on_toggle(on_change)
            .width(Length::Fill),
    )
}

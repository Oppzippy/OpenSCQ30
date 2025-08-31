use std::ops::RangeInclusive;

use cosmic::{Element, widget};
use openscq30_i18n::Translate;
use openscq30_lib::settings::SettingId;

use crate::device_settings::labeled_setting_row;

pub fn i32_range<'a, M>(
    setting_id: SettingId,
    range: RangeInclusive<i32>,
    value: i32,
    on_change: impl Fn(i32) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    labeled_setting_row(
        setting_id.translate(),
        widget::slider(range, value, on_change),
    )
}

use cosmic::{Element, widget};
use openscq30_i18n::Translate;
use openscq30_lib::settings::{self, SettingId};

use crate::device_settings::labeled_setting_row;

pub fn i32_range<'a, M>(
    setting_id: SettingId,
    range: settings::Range<i32>,
    value: i32,
    on_change: impl Fn(i32) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    labeled_setting_row(
        setting_id.translate(),
        widget::slider(range.range, value, on_change).step(range.step),
    )
}

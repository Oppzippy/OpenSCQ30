use std::ops::RangeInclusive;

use cosmic::{Element, widget};
use openscq30_lib::api::settings::SettingId;

pub fn i32_range<'a, M>(
    setting_id: SettingId<'static>,
    range: RangeInclusive<i32>,
    value: i32,
    on_change: impl Fn(i32) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    widget::row()
        .push(widget::text::body(setting_id.0))
        .push(widget::slider(range, value, on_change))
        .into()
}

use cosmic::{Element, iced::alignment, widget};
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
        widget::row::with_children([
            widget::text::body(value.to_string()).width(40).into(),
            widget::slider(range.range, value, on_change)
                .step(range.step)
                .into(),
        ])
        .align_y(alignment::Vertical::Center),
    )
}

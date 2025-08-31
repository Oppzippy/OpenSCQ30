use std::borrow::Cow;

use cosmic::{Element, widget};
use openscq30_i18n::Translate;
use openscq30_lib::settings::SettingId;

use crate::device_settings::labeled_setting_row;

pub fn input<M>(
    setting_id: SettingId,
    text: Cow<'_, str>,
    on_input: impl Fn(String) -> M + 'static,
    on_submit: impl Fn(String) -> M + 'static,
) -> Element<'_, M>
where
    M: Clone + 'static,
{
    labeled_setting_row(
        setting_id.translate(),
        widget::text_input("", text)
            .on_input(on_input)
            .on_submit(on_submit),
    )
}

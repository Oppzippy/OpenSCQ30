use cosmic::{Element, widget};
use openscq30_i18n::Translate;
use openscq30_lib::settings::SettingId;

use crate::{device_settings::labeled_setting_row, fl};

pub fn action<M>(setting_id: SettingId, on_execute: M) -> Element<'static, M>
where
    M: Clone + 'static,
{
    labeled_setting_row(
        setting_id.translate(),
        widget::button::standard(fl!("execute")).on_press(on_execute),
    )
}

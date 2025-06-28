use std::borrow::Cow;

use cosmic::{Element, widget};
use openscq30_i18n::Translate;
use openscq30_lib::api::settings::SettingId;

use crate::device_settings::labeled_setting_row;

pub fn information<'a, M>(setting_id: SettingId, text: Cow<'a, str>) -> Element<'a, M>
where
    M: Clone + 'static,
{
    labeled_setting_row(setting_id.translate(), widget::text(text))
}

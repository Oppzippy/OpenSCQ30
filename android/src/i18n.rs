use openscq30_i18n::Translate;

use crate::serializable;

#[uniffi::export]
pub fn translate_category_id(category_id: serializable::CategoryId) -> String {
    category_id.0.translate()
}

#[uniffi::export]
pub fn translate_setting_id(setting_id: serializable::SettingId) -> String {
    setting_id.0.translate()
}

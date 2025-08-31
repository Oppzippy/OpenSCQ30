use openscq30_i18n::Translate;
use openscq30_lib::settings;

use crate::serializable;

#[uniffi::export]
pub fn translate_category_id(category_id: serializable::CategoryId) -> String {
    category_id.0.translate()
}

#[uniffi::export]
pub fn translate_setting_id(setting_id: serializable::SettingId) -> String {
    setting_id.0.translate()
}

#[uniffi::export]
pub fn translate_device_model(device_model: serializable::DeviceModel) -> String {
    device_model.0.translate()
}

#[uniffi::export]
pub fn translate_value(
    setting: Option<serializable::Setting>,
    value: &serializable::Value,
) -> String {
    settings::localize_value(setting.map(|setting| setting.0).as_ref(), &value.0)
}

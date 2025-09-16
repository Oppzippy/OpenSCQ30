use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::common::{
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::{DualFirmwareVersion, SerialNumber},
    },
    i18n::fl,
};

use super::SerialNumberAndDualFirmwareVersionSetting;

#[derive(Default)]
pub struct SerialNumberAndFirmwareVersionSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for SerialNumberAndFirmwareVersionSettingHandler
where
    T: Has<SerialNumber> + Has<DualFirmwareVersion> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SerialNumberAndDualFirmwareVersionSetting::iter()
            .map(Into::into)
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let serial_number: &SerialNumber = state.get();
        let dual_firmware_version: &DualFirmwareVersion = state.get();
        let setting: SerialNumberAndDualFirmwareVersionSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            SerialNumberAndDualFirmwareVersionSetting::SerialNumber => Setting::Information {
                value: serial_number.to_string(),
                translated_value: serial_number.to_string(),
            },
            SerialNumberAndDualFirmwareVersionSetting::FirmwareVersionLeft => {
                let left_version = dual_firmware_version
                    .left()
                    .map(|version| version.to_string());
                Setting::Information {
                    value: left_version.clone().unwrap_or_default(),
                    translated_value: left_version.unwrap_or_else(|| fl!("none")),
                }
            }
            SerialNumberAndDualFirmwareVersionSetting::FirmwareVersionRight => {
                let right_version = dual_firmware_version
                    .right()
                    .map(|version| version.to_string());
                Setting::Information {
                    value: right_version.clone().unwrap_or_default(),
                    translated_value: right_version.unwrap_or_else(|| fl!("none")),
                }
            }
        })
    }

    async fn set(
        &self,
        _state: &mut T,
        _setting_id: &SettingId,
        _value: Value,
    ) -> SettingHandlerResult<()> {
        Err(SettingHandlerError::ReadOnly)
    }
}

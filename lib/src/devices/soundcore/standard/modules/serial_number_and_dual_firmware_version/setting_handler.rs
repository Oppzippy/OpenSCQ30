use async_trait::async_trait;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::standard::{
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::{DualFirmwareVersion, SerialNumber},
    },
};

use super::SerialNumberAndDualFirmwareVersionSetting;

#[derive(Default)]
pub struct SerialNumberAndFirmwareVersionSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for SerialNumberAndFirmwareVersionSettingHandler
where
    T: AsRef<SerialNumber> + AsRef<DualFirmwareVersion> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SerialNumberAndDualFirmwareVersionSetting::iter()
            .map(Into::into)
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let serial_number: &SerialNumber = state.as_ref();
        let dual_firmware_version: &DualFirmwareVersion = state.as_ref();
        let setting: SerialNumberAndDualFirmwareVersionSetting = setting_id.try_into().ok()?;
        Some(match setting {
            SerialNumberAndDualFirmwareVersionSetting::SerialNumber => Setting::Information {
                value: serial_number.to_string(),
                translated_value: serial_number.to_string(),
            },
            SerialNumberAndDualFirmwareVersionSetting::FirmwareVersionLeft => {
                Setting::Information {
                    value: dual_firmware_version.left.to_string(),
                    translated_value: dual_firmware_version.left.to_string(),
                }
            }
            SerialNumberAndDualFirmwareVersionSetting::FirmwareVersionRight => {
                Setting::Information {
                    value: dual_firmware_version.right.to_string(),
                    translated_value: dual_firmware_version.right.to_string(),
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

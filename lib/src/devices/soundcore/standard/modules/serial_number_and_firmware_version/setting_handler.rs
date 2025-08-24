use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::standard::{
        settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
        structures::{FirmwareVersion, SerialNumber},
    },
};

use super::SerialNumberAndFirmwareVersionSetting;

#[derive(Default)]
pub struct SerialNumberAndFirmwareVersionSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for SerialNumberAndFirmwareVersionSettingHandler
where
    T: Has<SerialNumber> + Has<FirmwareVersion> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SerialNumberAndFirmwareVersionSetting::iter()
            .map(Into::into)
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let serial_number: &SerialNumber = state.get();
        let firmware_version: &FirmwareVersion = state.get();
        let setting: SerialNumberAndFirmwareVersionSetting = setting_id.try_into().ok()?;
        Some(match setting {
            SerialNumberAndFirmwareVersionSetting::SerialNumber => Setting::Information {
                value: serial_number.to_string(),
                translated_value: serial_number.to_string(),
            },
            SerialNumberAndFirmwareVersionSetting::FirmwareVersion => Setting::Information {
                value: firmware_version.to_string(),
                translated_value: firmware_version.to_string(),
            },
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

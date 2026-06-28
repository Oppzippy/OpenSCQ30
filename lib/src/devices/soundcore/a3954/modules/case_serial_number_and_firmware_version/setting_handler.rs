use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3954,
        common::settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
    },
};

use super::SerialNumberAndFirmwareVersionSetting;

#[derive(Default)]
pub struct CaseSerialNumberAndFirmwareVersionSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for CaseSerialNumberAndFirmwareVersionSettingHandler
where
    T: Has<a3954::structures::CaseSerialNumber>
        + Has<a3954::structures::CaseFirmwareVersion>
        + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SerialNumberAndFirmwareVersionSetting::iter()
            .map(Into::into)
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let serial_number: &a3954::structures::CaseSerialNumber = state.get();
        let firmware_version: &a3954::structures::CaseFirmwareVersion = state.get();
        let setting: SerialNumberAndFirmwareVersionSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            SerialNumberAndFirmwareVersionSetting::CaseSerialNumber => Setting::Information {
                value: serial_number.to_string(),
                translated_value: serial_number.to_string(),
            },
            SerialNumberAndFirmwareVersionSetting::CaseFirmwareVersion => Setting::Information {
                value: firmware_version.0.to_string(),
                translated_value: firmware_version.0.to_string(),
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

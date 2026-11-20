use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        common::{
            settings_manager::{SettingHandler, SettingHandlerError, SettingHandlerResult},
            structures::SerialNumber,
        },
        d5103::structures::D5103FirmwareVersion,
    },
};

use super::DeviceInformationSetting;

pub struct DeviceInformationSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for DeviceInformationSettingHandler
where
    T: Has<SerialNumber> + Has<D5103FirmwareVersion> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        DeviceInformationSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let serial_number: &SerialNumber = state.get();
        let firmware_version: &D5103FirmwareVersion = state.get();
        let setting: DeviceInformationSetting = (*setting_id).try_into().ok()?;
        Some(match setting {
            DeviceInformationSetting::SerialNumber => Setting::Information {
                value: serial_number.to_string(),
                translated_value: serial_number.to_string(),
            },
            DeviceInformationSetting::FirmwareVersion => Setting::Information {
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

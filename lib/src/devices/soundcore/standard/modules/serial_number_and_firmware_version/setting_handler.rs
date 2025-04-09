use async_trait::async_trait;
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::standard::{
        settings_manager::SettingHandler,
        structures::{FirmwareVersion, SerialNumber},
    },
};

use super::SerialNumberAndFirmwareVersionSetting;

#[derive(Default)]
pub struct SerialNumberAndFirmwareVersionSettingHandler {}

#[async_trait]
impl<T> SettingHandler<T> for SerialNumberAndFirmwareVersionSettingHandler
where
    T: AsRef<SerialNumber> + AsRef<FirmwareVersion> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        SerialNumberAndFirmwareVersionSetting::iter()
            .map(Into::into)
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let setting: SerialNumberAndFirmwareVersionSetting = setting_id.try_into().ok()?;
        Some(match setting {
            SerialNumberAndFirmwareVersionSetting::SerialNumber => Setting::Information {
                text: <T as AsRef<SerialNumber>>::as_ref(state).to_string(),
            },
            SerialNumberAndFirmwareVersionSetting::FirmwareVersion => Setting::Information {
                text: <T as AsRef<FirmwareVersion>>::as_ref(state).to_string(),
            },
        })
    }

    async fn set(
        &self,
        _state: &mut T,
        _setting_id: &SettingId,
        _value: Value,
    ) -> crate::Result<()> {
        unimplemented!("serial number and firmware vesrion are read only")
    }
}

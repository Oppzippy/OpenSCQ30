use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::standard::structures::{FirmwareVersion, SerialNumber},
};

use super::ModuleCollection;

mod setting_handler;

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum SerialNumberAndFirmwareVersionSetting {
    SerialNumber,
    FirmwareVersion,
}

impl From<SerialNumberAndFirmwareVersionSetting> for SettingId {
    fn from(setting: SerialNumberAndFirmwareVersionSetting) -> Self {
        match setting {
            SerialNumberAndFirmwareVersionSetting::SerialNumber => SettingId::SerialNumber,
            SerialNumberAndFirmwareVersionSetting::FirmwareVersion => SettingId::FirmwareVersion,
        }
    }
}

impl TryFrom<&SettingId> for SerialNumberAndFirmwareVersionSetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::SerialNumber => Ok(Self::SerialNumber),
            SettingId::FirmwareVersion => Ok(Self::FirmwareVersion),
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: AsRef<SerialNumber> + AsRef<FirmwareVersion> + Clone + Send + Sync,
{
    pub fn add_serial_number_and_firmware_version(&mut self) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::SerialNumberAndFirmwareVersionSettingHandler::default(),
        );
    }
}

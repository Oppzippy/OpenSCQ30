use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::standard::structures::{DualFirmwareVersion, SerialNumber},
};

use super::ModuleCollection;

mod setting_handler;

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum SerialNumberAndDualFirmwareVersionSetting {
    SerialNumber,
    FirmwareVersionLeft,
    FirmwareVersionRight,
}

impl From<SerialNumberAndDualFirmwareVersionSetting> for SettingId {
    fn from(setting: SerialNumberAndDualFirmwareVersionSetting) -> Self {
        match setting {
            SerialNumberAndDualFirmwareVersionSetting::SerialNumber => SettingId::SerialNumber,
            SerialNumberAndDualFirmwareVersionSetting::FirmwareVersionLeft => {
                SettingId::FirmwareVersionLeft
            }
            SerialNumberAndDualFirmwareVersionSetting::FirmwareVersionRight => {
                SettingId::FirmwareVersionRight
            }
        }
    }
}

impl TryFrom<&SettingId> for SerialNumberAndDualFirmwareVersionSetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::SerialNumber => Ok(Self::SerialNumber),
            SettingId::FirmwareVersionLeft => Ok(Self::FirmwareVersionLeft),
            SettingId::FirmwareVersionRight => Ok(Self::FirmwareVersionRight),
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: AsRef<SerialNumber> + AsRef<DualFirmwareVersion> + Clone + Send + Sync,
{
    pub fn add_serial_number_and_dual_firmware_version(&mut self) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::SerialNumberAndFirmwareVersionSettingHandler::default(),
        );
    }
}

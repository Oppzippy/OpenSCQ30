use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::standard::structures::{DualFirmwareVersion, SerialNumber},
};

use super::ModuleCollection;

mod packet_handler;
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
            SerialNumberAndDualFirmwareVersionSetting::SerialNumber => Self::SerialNumber,
            SerialNumberAndDualFirmwareVersionSetting::FirmwareVersionLeft => {
                Self::FirmwareVersionLeft
            }
            SerialNumberAndDualFirmwareVersionSetting::FirmwareVersionRight => {
                Self::FirmwareVersionRight
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
    T: Has<SerialNumber> + Has<DualFirmwareVersion> + Clone + Send + Sync,
{
    pub fn add_serial_number_and_dual_firmware_version(&mut self) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::SerialNumberAndFirmwareVersionSettingHandler::default(),
        );
        self.packet_handlers.set_handler(
            packet_handler::SerialNumberAndDualFirmwareVersionPacketHandler::COMMAND,
            Box::new(packet_handler::SerialNumberAndDualFirmwareVersionPacketHandler::default()),
        );
    }
}

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::common::structures::{DualFirmwareVersion, SerialNumber},
    macros::enum_subset,
};

use super::ModuleCollection;

mod packet_handler;
mod setting_handler;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SerialNumberAndDualFirmwareVersionSetting {
        SerialNumber,
        FirmwareVersionLeft,
        FirmwareVersionRight,
    }
);

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

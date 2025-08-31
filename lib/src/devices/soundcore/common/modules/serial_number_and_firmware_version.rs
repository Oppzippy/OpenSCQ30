use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::common::structures::{FirmwareVersion, SerialNumber},
    macros::enum_subset,
};

use super::ModuleCollection;

mod setting_handler;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SerialNumberAndFirmwareVersionSetting {
        SerialNumber,
        FirmwareVersion,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<SerialNumber> + Has<FirmwareVersion> + Clone + Send + Sync,
{
    pub fn add_serial_number_and_firmware_version(&mut self) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::SerialNumberAndFirmwareVersionSettingHandler::default(),
        );
    }
}

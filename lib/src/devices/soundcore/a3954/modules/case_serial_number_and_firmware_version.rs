use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{a3954, common::modules::ModuleCollection},
    macros::enum_subset,
};

mod setting_handler;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SerialNumberAndFirmwareVersionSetting {
        CaseSerialNumber,
        CaseFirmwareVersion,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<a3954::structures::CaseSerialNumber>
        + Has<a3954::structures::CaseFirmwareVersion>
        + Send,
{
    pub fn add_a3954_case_serial_number_and_firmware_version(&mut self) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::CaseSerialNumberAndFirmwareVersionSettingHandler,
        );
    }
}

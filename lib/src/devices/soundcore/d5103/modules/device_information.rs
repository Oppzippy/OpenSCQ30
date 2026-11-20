use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        common::{modules::ModuleCollection, structures::SerialNumber},
        d5103::structures::D5103FirmwareVersion,
    },
    macros::enum_subset,
};

mod setting_handler;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum DeviceInformationSetting {
        SerialNumber,
        FirmwareVersion,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<SerialNumber> + Has<D5103FirmwareVersion> + Clone + Send + Sync,
{
    pub fn add_d5103_device_information(&mut self) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::DeviceInformationSettingHandler,
        );
    }
}

mod setting_handler;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::common::structures::CaseBatteryLevel,
    macros::enum_subset,
};

use super::ModuleCollection;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum CaseBatteryLevelSetting {
        CaseBatteryLevel,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<CaseBatteryLevel> + Clone + Send + Sync,
{
    pub fn add_case_battery_level(&mut self, max_level: u8) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::CaseBatteryLevelSettingHandler::new(max_level),
        );
    }
}

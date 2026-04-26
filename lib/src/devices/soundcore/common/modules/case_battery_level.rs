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

#[derive(Debug, Default, Copy, Clone)]
pub struct CaseBatteryLevelConfiguration {
    /// This is the max level after applying the offset
    pub max_level: u8,
    /// Applied to the level before dividing by max_level. This makes each level
    /// behaves as level+offset, so if 1, 0 will represent 1, 1 will be 2, etc.
    pub level_offset: u8,
}

impl<T> ModuleCollection<T>
where
    T: Has<CaseBatteryLevel> + Clone + Send + Sync,
{
    pub fn add_case_battery_level(&mut self, configuration: CaseBatteryLevelConfiguration) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::CaseBatteryLevelSettingHandler::new(&configuration),
        );
    }
}

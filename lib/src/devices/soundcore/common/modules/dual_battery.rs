mod packet_handler;
mod setting_handler;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::common::structures::DualBattery,
    macros::enum_subset,
};

use super::ModuleCollection;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum BatterySetting {
        IsChargingLeft,
        IsChargingRight,
        BatteryLevelLeft,
        BatteryLevelRight,
    }
);

#[derive(Debug, Default, Copy, Clone)]
pub struct DualBatteryConfiguration {
    /// This is the max level after applying the offset
    pub max_level: u8,
    /// Applied to the level before dividing by max_level. This makes each level
    /// behaves as level+offset, so if 1, 0 will represent 1, 1 will be 2, etc.
    pub level_offset: u8,
}

impl<T> ModuleCollection<T>
where
    T: Has<DualBattery> + Clone + Send + Sync,
{
    pub fn add_dual_battery(&mut self, configuration: DualBatteryConfiguration) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::BatterySettingHandler::new(&configuration),
        );
        self.packet_handlers.set_handler(
            packet_handler::BatteryLevelPacketHandler::COMMAND,
            Box::new(packet_handler::BatteryLevelPacketHandler::default()),
        );
        self.packet_handlers.set_handler(
            packet_handler::BatteryChargingPacketHandler::COMMAND,
            Box::new(packet_handler::BatteryChargingPacketHandler::default()),
        );
    }
}

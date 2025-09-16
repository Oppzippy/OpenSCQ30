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

impl<T> ModuleCollection<T>
where
    T: Has<DualBattery> + Clone + Send + Sync,
{
    pub fn add_dual_battery(&mut self, max_level: u8) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::BatterySettingHandler::new(max_level),
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

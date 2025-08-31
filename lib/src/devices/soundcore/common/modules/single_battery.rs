mod packet_handler;
mod setting_handler;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::common::structures::SingleBattery,
    macros::enum_subset,
};

use super::ModuleCollection;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum BatterySetting {
        IsCharging,
        BatteryLevel,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<SingleBattery> + Clone + Send + Sync,
{
    pub fn add_single_battery(&mut self) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::BatterySettingHandler::default(),
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

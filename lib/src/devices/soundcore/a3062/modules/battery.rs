mod setting_handler;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3062,
        common::{self, modules::ModuleCollection, structures::SingleBattery},
    },
    macros::enum_subset,
};

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
    pub fn add_a3062_battery(&mut self, max_level: u8) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            a3062::modules::battery::setting_handler::BatterySettingHandler::new(max_level),
        );
        self.packet_handlers.set_handler(
            common::modules::single_battery::packet_handler::BatteryLevelPacketHandler::COMMAND,
            Box::new(
                common::modules::single_battery::packet_handler::BatteryLevelPacketHandler::default(
                ),
            ),
        );
        self.packet_handlers.set_handler(
            common::modules::single_battery::packet_handler::BatteryChargingPacketHandler::COMMAND,
            Box::new(
                common::modules::single_battery::packet_handler::BatteryChargingPacketHandler::default(),
            ),
        );
    }
}

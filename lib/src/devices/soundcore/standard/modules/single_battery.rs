mod packet_handler;
mod setting_handler;

use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::standard::structures::SingleBattery,
};

use super::ModuleCollection;

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum BatterySetting {
    IsCharging,
    BatteryLevel,
}

impl From<BatterySetting> for SettingId {
    fn from(setting: BatterySetting) -> Self {
        match setting {
            BatterySetting::IsCharging => Self::IsCharging,
            BatterySetting::BatteryLevel => Self::BatteryLevel,
        }
    }
}

impl TryFrom<&SettingId> for BatterySetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::IsCharging => Ok(Self::IsCharging),
            SettingId::BatteryLevel => Ok(Self::BatteryLevel),
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: AsMut<SingleBattery> + AsRef<SingleBattery> + Clone + Send + Sync,
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

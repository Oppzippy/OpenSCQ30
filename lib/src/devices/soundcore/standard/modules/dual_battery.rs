mod packet_handler;
mod setting_handler;

use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::standard::structures::DualBattery,
};

use super::ModuleCollection;

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum BatterySetting {
    IsChargingLeft,
    IsChargingRight,
    BatteryLevelLeft,
    BatteryLevelRight,
}

impl From<BatterySetting> for SettingId {
    fn from(setting: BatterySetting) -> Self {
        match setting {
            BatterySetting::IsChargingLeft => SettingId::IsChargingLeft,
            BatterySetting::BatteryLevelLeft => SettingId::BatteryLevelLeft,
            BatterySetting::IsChargingRight => SettingId::IsChargingRight,
            BatterySetting::BatteryLevelRight => SettingId::BatteryLevelRight,
        }
    }
}

impl TryFrom<&SettingId> for BatterySetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::IsChargingLeft => Ok(Self::IsChargingLeft),
            SettingId::BatteryLevelLeft => Ok(Self::BatteryLevelLeft),
            SettingId::IsChargingRight => Ok(Self::IsChargingRight),
            SettingId::BatteryLevelRight => Ok(Self::BatteryLevelRight),
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: AsMut<DualBattery> + AsRef<DualBattery> + Clone + Send + Sync,
{
    pub fn add_dual_battery(&mut self) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::AmbientBatteryCycleSettingHandler::default(),
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

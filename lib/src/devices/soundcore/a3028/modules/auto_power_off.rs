use std::sync::Arc;

use strum::{EnumIter, EnumString};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::{
        a3028::{
            modules::auto_power_off::{
                setting_handler::AutoPowerOffSettingHandler,
                state_modifier::AutoPowerOffStateModifier,
            },
            packets::AutoPowerOff,
        },
        standard::{modules::ModuleCollection, packet::packet_io_controller::PacketIOController},
    },
};

mod setting_handler;
mod state_modifier;

#[derive(EnumIter, EnumString)]
enum AutoPowerOffSetting {
    AutoPowerOff,
}

impl From<AutoPowerOffSetting> for SettingId {
    fn from(value: AutoPowerOffSetting) -> Self {
        match value {
            AutoPowerOffSetting::AutoPowerOff => Self::AutoPowerOff,
        }
    }
}

impl TryFrom<SettingId> for AutoPowerOffSetting {
    type Error = ();

    fn try_from(setting_id: SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::AutoPowerOff => Ok(Self::AutoPowerOff),
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: AsMut<Option<AutoPowerOff>> + AsRef<Option<AutoPowerOff>> + Clone + Send + Sync,
{
    pub fn add_auto_power_off<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, AutoPowerOffSettingHandler::new());
        self.state_modifiers
            .push(Box::new(AutoPowerOffStateModifier::new(packet_io)));
    }
}

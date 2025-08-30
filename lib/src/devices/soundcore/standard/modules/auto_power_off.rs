use std::sync::Arc;

use openscq30_i18n::Translate;
use openscq30_lib_has::MaybeHas;
use strum::{EnumIter, EnumString};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::standard::{
        modules::{
            ModuleCollection,
            auto_power_off::{
                setting_handler::AutoPowerOffSettingHandler,
                state_modifier::AutoPowerOffStateModifier,
            },
        },
        packet::packet_io_controller::PacketIOController,
        structures::AutoPowerOff,
    },
    macros::enum_subset,
};

mod setting_handler;
mod state_modifier;

enum_subset!(
    SettingId,
    #[derive(EnumIter, EnumString)]
    enum AutoPowerOffSetting {
        AutoPowerOff,
    }
);

impl<T> ModuleCollection<T>
where
    T: MaybeHas<AutoPowerOff> + Clone + Send + Sync + 'static,
{
    pub fn add_auto_power_off<C, Duration>(
        &mut self,
        packet_io: Arc<PacketIOController<C>>,
        durations: &'static [Duration],
    ) where
        C: RfcommConnection + 'static + Send + Sync,
        Duration: Translate + Send + Sync + 'static,
        &'static str: for<'a> From<&'a Duration>,
    {
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            AutoPowerOffSettingHandler::new(durations),
        );
        self.state_modifiers
            .push(Box::new(AutoPowerOffStateModifier::new(packet_io)));
    }
}

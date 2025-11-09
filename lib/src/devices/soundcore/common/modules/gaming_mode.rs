mod packet_handler;
mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    connection::RfcommConnection,
    devices::soundcore::common::{packet::PacketIOController, structures::GamingMode},
    macros::enum_subset,
};

use super::ModuleCollection;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum GamingModeSetting {
        GamingMode,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<GamingMode> + Clone + Send + Sync,
{
    pub fn add_gaming_mode<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.packet_handlers.set_handler(
            packet_handler::GamingModePacketHandler::COMMAND,
            Box::new(packet_handler::GamingModePacketHandler::default()),
        );
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::GamingModeSettingHandler::default(),
        );
        self.state_modifiers
            .push(Box::new(state_modifier::GamingModeStateModifier::new(
                packet_io,
            )));
    }
}

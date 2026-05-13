use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString};

use crate::{
    connection::RfcommConnection,
    devices::soundcore::common::{
        modules::ModuleCollection, packet::PacketIOController, structures::DualConnections,
    },
    macros::enum_subset,
    settings::{CategoryId, SettingId},
};

mod packet_handler;
mod setting_handler;
mod state_modifier;

enum_subset! {
    SettingId,
    #[derive(EnumIter, EnumString)]
    enum DualConnectionsSetting {
        DualConnections,
        DualConnectionsDevices,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<DualConnections> + Clone + Send + Sync,
{
    pub fn add_dual_connections<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.packet_handlers.set_handler(
            packet_handler::DualConnectionsDevicePacketHandler::COMMAND,
            Box::new(packet_handler::DualConnectionsDevicePacketHandler),
        );
        self.setting_manager.add_handler(
            CategoryId::DualConnections,
            setting_handler::DualConnectionsSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(state_modifier::DualConnectionsStateModifier::new(
                packet_io,
            )));
    }
}

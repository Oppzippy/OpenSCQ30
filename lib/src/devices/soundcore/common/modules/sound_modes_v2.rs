mod migration_planner;
mod migration_state_modifier;
mod packet_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;

pub use migration_planner::*;

use crate::devices::soundcore::common::{
    modules::ModuleCollection,
    packet::{self, PacketIOController, inbound::FromPacketBody},
};

impl<StateT> ModuleCollection<StateT> {
    pub fn add_partial_sound_modes_v2<SoundModesT>(&mut self, packet_io: Arc<PacketIOController>)
    where
        StateT: Has<SoundModesT> + Send + Sync,
        SoundModesT: FromPacketBody<DirectionMarker = packet::InboundMarker>
            + ToPacketBody
            + Clone
            + PartialEq
            + Send
            + Sync
            + 'static,
    {
        self.packet_handlers.set_handler(
            packet_handler::PACKET_HANDLER_COMMAND,
            Box::new(packet_handler::SoundModesPacketHandler::default()),
        );
        self.state_modifiers
            .push(Box::new(state_modifier::SoundModesStateModifier::new(
                packet_io,
            )));
    }

    /// Some devices don't like when you make sound mode state transitions that the Soundcore app
    /// doesn't do. This will go through the steps to get from state A to state B.
    pub fn add_partial_sound_modes_v2_with_migration<
        SoundModesT,
        SoundModesFieldT,
        const SIZE: usize,
    >(
        &mut self,
        packet_io: Arc<PacketIOController>,
    ) where
        StateT: Has<SoundModesT> + Send + Sync,
        SoundModesT: FromPacketBody<DirectionMarker = packet::InboundMarker>
            + Migrate<SIZE, T = SoundModesFieldT>
            + ToPacketBody
            + std::fmt::Debug
            + PartialEq
            + Send
            + Sync
            + 'static,
        SoundModesFieldT: Send + Sync + 'static,
    {
        self.packet_handlers.set_handler(
            packet_handler::PACKET_HANDLER_COMMAND,
            Box::new(packet_handler::SoundModesPacketHandler::default()),
        );
        self.state_modifiers.push(Box::new(
            migration_state_modifier::SoundModesStateModifier::new(packet_io),
        ));
    }
}

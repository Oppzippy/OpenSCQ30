mod migration_planner;
mod packet_handler;
mod state_modifier;
mod sync_packet_handler;
mod sync_state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;

pub use migration_planner::*;
use tokio::sync::Notify;

use crate::devices::soundcore::common::{
    modules::ModuleCollection,
    packet::{self, PacketIOController, inbound::FromPacketBody},
};

impl<StateT> ModuleCollection<StateT> {
    /// If this changes sound modes faster than the device can handle, consider using
    /// `add_partial_sound_modes_v2_with_sound_mode_update_lock`.
    pub fn add_partial_sound_modes_v2<SoundModesT, SoundModesFieldT, const SIZE: usize>(
        &mut self,
        packet_io: Arc<PacketIOController>,
    ) where
        StateT: Has<SoundModesT> + Clone + Send + Sync,
        SoundModesT: FromPacketBody<DirectionMarker = packet::InboundMarker>
            + Migrate<SIZE, T = SoundModesFieldT>
            + ToPacketBody
            + std::fmt::Debug
            + Default
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
        self.state_modifiers
            .push(Box::new(state_modifier::SoundModesStateModifier::new(
                packet_io,
            )));
    }

    /// After we send a set sound modes packet, we will wait for a sound mode update packet before
    /// continuing. Some devices will not process set sound mode packets between when one is sent to
    /// the device and when the following sound mode update is received by us.
    ///
    /// This significantly slows down the speed at which we can change many sound modes, so it's
    /// unfortunate that this is necessary.
    ///
    /// This sequence of events should demonstrate the use of this flag
    /// 1. Send set sound modes to Space A40
    /// 2. Space A40 receives command and replies with ACK. set sound mode packets are now ignored.
    /// 3. Send another set sound modes to Space A40
    /// 4. Space A40 receives set sound modes, but ignores it
    /// 5. Space A40 sends sound mode update and is no longer ignoring set sound modes
    pub fn add_partial_sound_modes_v2_with_sound_mode_update_lock<
        SoundModesT,
        SoundModesFieldT,
        const SIZE: usize,
    >(
        &mut self,
        packet_io: Arc<PacketIOController>,
    ) where
        StateT: Has<SoundModesT> + Clone + Send + Sync,
        SoundModesT: FromPacketBody<DirectionMarker = packet::InboundMarker>
            + Migrate<SIZE, T = SoundModesFieldT>
            + ToPacketBody
            + std::fmt::Debug
            + Default
            + PartialEq
            + Send
            + Sync
            + 'static,
        SoundModesFieldT: Send + Sync + 'static,
    {
        let notify = Arc::new(Notify::new());
        self.packet_handlers.set_handler(
            sync_packet_handler::PACKET_HANDLER_COMMAND,
            Box::new(sync_packet_handler::SynchronizedSoundModesPacketHandler::new(notify.clone())),
        );
        self.state_modifiers.push(Box::new(
            sync_state_modifier::SynchronizedSoundModesStateModifier::new(packet_io, notify),
        ));
    }
}

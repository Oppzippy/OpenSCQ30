use std::marker::PhantomData;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{
            self,
            inbound::{FromPacketBody, TryToPacket},
        },
        packet_manager::PacketHandler,
    },
};

pub const PACKET_HANDLER_COMMAND: packet::Command = packet::Command([0x06, 0x01]);

pub struct SoundModesPacketHandler<SoundModesT> {
    _sound_modes: PhantomData<SoundModesT>,
}

impl<SoundModesT> Default for SoundModesPacketHandler<SoundModesT> {
    fn default() -> Self {
        Self {
            _sound_modes: PhantomData,
        }
    }
}

#[async_trait]
impl<StateT, SoundModesT> PacketHandler<StateT> for SoundModesPacketHandler<SoundModesT>
where
    StateT: Has<SoundModesT> + Send + Sync,
    SoundModesT: FromPacketBody<DirectionMarker = packet::InboundMarker> + PartialEq + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<StateT>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let new_sound_modes: SoundModesT = packet.try_to_packet()?;
        state.send_if_modified(|state| {
            let sound_modes = state.get_mut();
            let modified = new_sound_modes == *sound_modes;
            *sound_modes = new_sound_modes;
            modified
        });
        Ok(())
    }
}

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3936::{packets::A3936SoundModesUpdatePacket, structures::A3936SoundModes},
        common::{
            packet::{self, Command, inbound::TryIntoPacket},
            packet_manager::PacketHandler,
        },
    },
};

#[derive(Default)]
pub struct SoundModesPacketHandler {}

impl SoundModesPacketHandler {
    pub const COMMAND: Command = A3936SoundModesUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for SoundModesPacketHandler
where
    T: Has<A3936SoundModes> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3936SoundModesUpdatePacket = packet.try_into_packet()?;
        state.send_if_modified(|state| {
            let sound_modes = state.get_mut();
            let modified = packet.sound_modes == *sound_modes;
            *sound_modes = packet.sound_modes;
            modified
        });
        Ok(())
    }
}

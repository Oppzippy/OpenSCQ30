use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3959,
        common::{
            packet::{self, inbound::TryToPacket},
            packet_manager::PacketHandler,
        },
    },
};

#[derive(Default)]
pub struct SoundModesPacketHandler {}

impl SoundModesPacketHandler {
    pub const COMMAND: packet::Command = a3959::packets::inbound::A3959SoundModes::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for SoundModesPacketHandler
where
    T: Has<a3959::structures::SoundModes> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: a3959::packets::inbound::A3959SoundModes = packet.try_to_packet()?;
        state.send_if_modified(|state| {
            let sound_modes = state.get_mut();
            let modified = packet.sound_modes == *sound_modes;
            *sound_modes = packet.sound_modes;
            modified
        });
        Ok(())
    }
}

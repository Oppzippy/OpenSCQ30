use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3959::{packets::A3959SoundModesUpdatePacket, structures::A3959SoundModes},
        standard::{
            packet_manager::PacketHandler,
            packet::{Command, Packet, inbound::TryIntoInboundPacket},
        },
    },
};

#[derive(Default)]
pub struct SoundModesPacketHandler {}

impl SoundModesPacketHandler {
    pub const COMMAND: Command = A3959SoundModesUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for SoundModesPacketHandler
where
    T: AsMut<A3959SoundModes> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> device::Result<()> {
        let packet: A3959SoundModesUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let sound_modes = state.as_mut();
            let modified = packet.sound_modes == *sound_modes;
            *sound_modes = packet.sound_modes;
            modified
        });
        Ok(())
    }
}

use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    devices::{
        a3936::{packets::A3936SoundModesUpdatePacket, structures::A3936SoundModes},
        standard::{
            packet_manager::PacketHandler, packets::inbound::TryIntoInboundPacket,
            structures::Command,
        },
    },
    soundcore_device::device::Packet,
};

#[derive(Default)]
pub struct SoundModesPacketHandler {}

impl SoundModesPacketHandler {
    pub const COMMAND: Command = A3936SoundModesUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for SoundModesPacketHandler
where
    T: AsMut<A3936SoundModes> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> crate::Result<()> {
        let packet: A3936SoundModesUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let sound_modes = state.as_mut();
            let modified = packet.sound_modes == *sound_modes;
            *sound_modes = packet.sound_modes;
            modified
        });
        Ok(())
    }
}

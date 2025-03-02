use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    devices::standard::{
        packet_manager::PacketHandler,
        packets::inbound::{SoundModeUpdatePacket, TryIntoInboundPacket},
        structures::{Command, SoundModes},
    },
    soundcore_device::device::Packet,
};

#[derive(Default)]
pub struct SoundModesPacketHandler {}

impl SoundModesPacketHandler {
    pub const COMMAND: Command = SoundModeUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for SoundModesPacketHandler
where
    T: AsMut<SoundModes> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> crate::Result<()> {
        let packet: SoundModeUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let sound_modes = state.as_mut();
            let modified = packet.0 == *sound_modes;
            *sound_modes = packet.0;
            modified
        });
        Ok(())
    }
}

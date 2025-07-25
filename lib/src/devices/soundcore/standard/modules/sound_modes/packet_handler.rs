use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::standard::{
        packet::{
            Command, Packet,
            inbound::{SoundModeUpdatePacket, TryIntoInboundPacket},
        },
        packet_manager::PacketHandler,
        structures::SoundModes,
    },
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
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> device::Result<()> {
        let packet: SoundModeUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let sound_modes = state.as_mut();
            let modified = packet.0 != *sound_modes;
            *sound_modes = packet.0;
            modified
        });
        Ok(())
    }
}

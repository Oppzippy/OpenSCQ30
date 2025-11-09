use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{self, Command, inbound::TryToPacket},
        packet_manager::PacketHandler,
        structures::GamingMode,
    },
};

#[derive(Default)]
pub struct GamingModePacketHandler;

impl GamingModePacketHandler {
    pub const COMMAND: Command = packet::inbound::GamingModeUpdate::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for GamingModePacketHandler
where
    T: Has<GamingMode> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: packet::inbound::GamingModeUpdate = packet.try_to_packet()?;
        state.send_if_modified(|state| {
            let gaming_mode = state.get_mut();
            let modified = packet.0 != *gaming_mode;
            *gaming_mode = packet.0;
            modified
        });
        Ok(())
    }
}

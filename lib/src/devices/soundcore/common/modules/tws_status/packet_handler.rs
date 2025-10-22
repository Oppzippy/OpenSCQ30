use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{self, inbound::TryIntoPacket},
        packet_manager::PacketHandler,
        structures,
    },
};

#[derive(Default)]
pub struct TwsStatusPacketHandler {}

impl TwsStatusPacketHandler {
    pub const COMMAND: packet::Command = packet::inbound::TwsStatus::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for TwsStatusPacketHandler
where
    T: Has<structures::TwsStatus> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: packet::inbound::TwsStatus = packet.try_into_packet()?;
        state.send_if_modified(|state| {
            let tws_status = state.get_mut();
            let modified = *tws_status != packet.0;
            *tws_status = packet.0;
            modified
        });
        Ok(())
    }
}

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{
            Command, Packet,
            inbound::{TryIntoInboundPacket, TwsStatusUpdatePacket},
        },
        packet_manager::PacketHandler,
        structures::TwsStatus,
    },
};

#[derive(Default)]
pub struct TwsStatusPacketHandler {}

impl TwsStatusPacketHandler {
    pub const COMMAND: Command = TwsStatusUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for TwsStatusPacketHandler
where
    T: Has<TwsStatus> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> device::Result<()> {
        let packet: TwsStatusUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let tws_status = state.get_mut();
            let modified = *tws_status != packet.0;
            *tws_status = packet.0;
            modified
        });
        Ok(())
    }
}

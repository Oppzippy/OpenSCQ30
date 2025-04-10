use async_trait::async_trait;
use tokio::sync::watch;

use crate::devices::soundcore::standard::{
    packet_manager::PacketHandler,
    packets::{
        Packet,
        inbound::{TryIntoInboundPacket, TwsStatusUpdatePacket},
    },
    structures::{Command, TwsStatus},
};

#[derive(Default)]
pub struct TwsStatusPacketHandler {}

impl TwsStatusPacketHandler {
    pub const COMMAND: Command = TwsStatusUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for TwsStatusPacketHandler
where
    T: AsMut<TwsStatus> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> crate::Result<()> {
        let packet: TwsStatusUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let tws_status = state.as_mut();
            let modified = *tws_status != packet.0;
            *tws_status = packet.0;
            modified
        });
        Ok(())
    }
}

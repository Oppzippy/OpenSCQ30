use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        common::{
            packet::{self, Command, inbound::TryToPacket},
            packet_manager::PacketHandler,
        },
        d1301::{packets::inbound::AutoStopTimerPacket, structures::AutoStopTimer},
    },
};

/// Handles the unsolicited 21 3 the device pushes when the auto stop timer or
/// the post-sleep audio action changes. Without it the packet is logged as
/// unhandled and the change is not seen until the next connect.
#[derive(Default)]
pub struct AutoStopTimerPacketHandler;

impl AutoStopTimerPacketHandler {
    pub const COMMAND: Command = AutoStopTimerPacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for AutoStopTimerPacketHandler
where
    T: Has<AutoStopTimer> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: AutoStopTimerPacket = packet.try_to_packet()?;
        state.send_if_modified(|state| {
            let auto_stop_timer: &mut AutoStopTimer = state.get_mut();
            let modified = *auto_stop_timer != packet.0;
            *auto_stop_timer = packet.0;
            modified
        });
        Ok(())
    }
}

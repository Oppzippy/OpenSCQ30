use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{self, Command, inbound::TryToPacket},
        packet_manager::PacketHandler,
        structures::DualBatteryLevel,
    },
};

#[derive(Default)]
pub struct BatteryLevelPacketHandler;

impl BatteryLevelPacketHandler {
    pub const COMMAND: Command = packet::inbound::DualBatteryLevel::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for BatteryLevelPacketHandler
where
    T: Has<DualBatteryLevel> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: packet::inbound::DualBatteryLevel = packet.try_to_packet()?;
        state.send_if_modified(|state| {
            let battery = state.get_mut();
            let modified = packet.left != battery.left || packet.right != battery.right;
            battery.left = packet.left;
            battery.right = packet.right;
            modified
        });
        Ok(())
    }
}

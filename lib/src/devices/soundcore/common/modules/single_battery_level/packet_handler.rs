use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{self, Command, inbound::TryToPacket},
        packet_manager::PacketHandler,
        structures::BatteryLevel,
    },
};

#[derive(Default)]
pub struct BatteryLevelPacketHandler;

impl BatteryLevelPacketHandler {
    pub const COMMAND: Command = packet::inbound::SingleBatteryLevel::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for BatteryLevelPacketHandler
where
    T: Has<BatteryLevel> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: packet::inbound::SingleBatteryLevel = packet.try_to_packet()?;
        state.send_if_modified(|state| {
            let battery = state.get_mut();
            let modified = packet.level != *battery;
            *battery = packet.level;
            modified
        });
        Ok(())
    }
}

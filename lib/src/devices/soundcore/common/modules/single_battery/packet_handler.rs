use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{self, Command, inbound::TryIntoPacket},
        packet_manager::PacketHandler,
        structures::SingleBattery,
    },
};

#[derive(Default)]
pub struct BatteryLevelPacketHandler {}

impl BatteryLevelPacketHandler {
    pub const COMMAND: Command = packet::inbound::SingleBatteryLevel::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for BatteryLevelPacketHandler
where
    T: Has<SingleBattery> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: packet::inbound::SingleBatteryLevel = packet.try_into_packet()?;
        state.send_if_modified(|state| {
            let battery = state.get_mut();
            let modified = packet.level != battery.level;
            battery.level = packet.level;
            modified
        });
        Ok(())
    }
}

#[derive(Default)]
pub struct BatteryChargingPacketHandler {}

impl BatteryChargingPacketHandler {
    pub const COMMAND: Command = packet::inbound::SingleBatteryCharging::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for BatteryChargingPacketHandler
where
    T: Has<SingleBattery> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: packet::inbound::SingleBatteryCharging = packet.try_into_packet()?;
        state.send_if_modified(|state| {
            let battery = state.get_mut();
            let modified = packet.is_charging != battery.is_charging;
            battery.is_charging = packet.is_charging;
            modified
        });
        Ok(())
    }
}

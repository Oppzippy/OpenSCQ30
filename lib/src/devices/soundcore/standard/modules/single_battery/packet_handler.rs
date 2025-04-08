use async_trait::async_trait;
use tokio::sync::watch;

use crate::devices::soundcore::standard::{
    packet_manager::PacketHandler,
    packets::{
        Packet,
        inbound::{BatteryChargingUpdatePacket, BatteryLevelUpdatePacket, TryIntoInboundPacket},
    },
    structures::{Command, SingleBattery},
};

#[derive(Default)]
pub struct BatteryLevelPacketHandler {}

impl BatteryLevelPacketHandler {
    pub const COMMAND: Command = BatteryLevelUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for BatteryLevelPacketHandler
where
    T: AsMut<SingleBattery> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> crate::Result<()> {
        let packet: BatteryLevelUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let battery = state.as_mut();
            let modified = packet.left == battery.level;
            battery.level = packet.left;
            modified
        });
        Ok(())
    }
}

#[derive(Default)]
pub struct BatteryChargingPacketHandler {}

impl BatteryChargingPacketHandler {
    pub const COMMAND: Command = BatteryChargingUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for BatteryChargingPacketHandler
where
    T: AsMut<SingleBattery> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> crate::Result<()> {
        let packet: BatteryChargingUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let battery = state.as_mut();
            let modified = packet.left == battery.is_charging;
            battery.is_charging = packet.left;
            modified
        });
        Ok(())
    }
}

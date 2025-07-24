use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::standard::{
        packet_manager::PacketHandler,
        packet::{
            Command, Packet,
            inbound::{
                SingleBatteryChargingUpdatePacket, SingleBatteryLevelUpdatePacket,
                TryIntoInboundPacket,
            },
        },
        structures::SingleBattery,
    },
};

#[derive(Default)]
pub struct BatteryLevelPacketHandler {}

impl BatteryLevelPacketHandler {
    pub const COMMAND: Command = SingleBatteryLevelUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for BatteryLevelPacketHandler
where
    T: AsMut<SingleBattery> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> device::Result<()> {
        let packet: SingleBatteryLevelUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let battery = state.as_mut();
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
    pub const COMMAND: Command = SingleBatteryChargingUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for BatteryChargingPacketHandler
where
    T: AsMut<SingleBattery> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> device::Result<()> {
        let packet: SingleBatteryChargingUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let battery = state.as_mut();
            let modified = packet.is_charging != battery.is_charging;
            battery.is_charging = packet.is_charging;
            modified
        });
        Ok(())
    }
}

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{
            Command, Packet,
            inbound::{DualBatteryCharging, DualBatteryLevel, TryIntoInboundPacket},
        },
        packet_manager::PacketHandler,
        structures::DualBattery,
    },
};

#[derive(Default)]
pub struct BatteryLevelPacketHandler {}

impl BatteryLevelPacketHandler {
    pub const COMMAND: Command = DualBatteryLevel::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for BatteryLevelPacketHandler
where
    T: Has<DualBattery> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> device::Result<()> {
        let packet: DualBatteryLevel = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let battery = state.get_mut();
            let modified = packet.left != battery.left.level || packet.right != battery.right.level;
            battery.left.level = packet.left;
            battery.right.level = packet.right;
            modified
        });
        Ok(())
    }
}

#[derive(Default)]
pub struct BatteryChargingPacketHandler {}

impl BatteryChargingPacketHandler {
    pub const COMMAND: Command = DualBatteryCharging::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for BatteryChargingPacketHandler
where
    T: Has<DualBattery> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> device::Result<()> {
        let packet: DualBatteryCharging = packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let battery = state.get_mut();
            let modified = packet.left != battery.left.is_charging
                || packet.right != battery.right.is_charging;
            battery.left.is_charging = packet.left;
            battery.right.is_charging = packet.right;
            modified
        });
        Ok(())
    }
}

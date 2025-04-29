use async_trait::async_trait;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::standard::{
        packet_manager::PacketHandler,
        packets::{
            Packet,
            inbound::{SerialNumberAndFirmwareVersionUpdatePacket, TryIntoInboundPacket},
        },
        structures::{Command, DualFirmwareVersion, SerialNumber},
    },
};

#[derive(Default)]
pub struct SerialNumberAndDualFirmwareVersionPacketHandler {}

impl SerialNumberAndDualFirmwareVersionPacketHandler {
    pub const COMMAND: Command = SerialNumberAndFirmwareVersionUpdatePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for SerialNumberAndDualFirmwareVersionPacketHandler
where
    T: AsMut<SerialNumber>
        + AsRef<SerialNumber>
        + AsMut<DualFirmwareVersion>
        + AsRef<DualFirmwareVersion>
        + Send
        + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> device::Result<()> {
        let packet: SerialNumberAndFirmwareVersionUpdatePacket =
            packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let modified = {
                let serial_number: &SerialNumber = state.as_ref();
                let dual_firmware_version: &DualFirmwareVersion = state.as_ref();
                packet.serial_number != *serial_number
                    || packet.dual_firmware_version != *dual_firmware_version
            };
            *state.as_mut() = packet.serial_number;
            *state.as_mut() = packet.dual_firmware_version;
            modified
        });
        Ok(())
    }
}

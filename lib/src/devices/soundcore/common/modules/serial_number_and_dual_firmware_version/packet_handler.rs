use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{
            Command, Packet,
            inbound::{SerialNumberAndFirmwareVersionUpdatePacket, TryIntoInboundPacket},
        },
        packet_manager::PacketHandler,
        structures::{DualFirmwareVersion, SerialNumber},
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
    T: Has<SerialNumber> + Has<DualFirmwareVersion> + Send + Sync,
{
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> device::Result<()> {
        let packet: SerialNumberAndFirmwareVersionUpdatePacket =
            packet.try_into_inbound_packet()?;
        state.send_if_modified(|state| {
            let modified = {
                let serial_number: &SerialNumber = state.get();
                let dual_firmware_version: &DualFirmwareVersion = state.get();
                packet.serial_number != *serial_number
                    || packet.dual_firmware_version != *dual_firmware_version
            };
            *state.get_mut() = packet.serial_number;
            *state.get_mut() = packet.dual_firmware_version;
            modified
        });
        Ok(())
    }
}

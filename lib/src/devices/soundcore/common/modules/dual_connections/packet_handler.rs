use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{self, Command, inbound::TryToPacket},
        packet_manager::PacketHandler,
        structures::DualConnections,
    },
};

#[derive(Default)]
pub struct DualConnectionsDevicePacketHandler;

impl DualConnectionsDevicePacketHandler {
    pub const COMMAND: Command = packet::inbound::DualConnectionsDevicePacket::COMMAND;
}

#[async_trait]
impl<T> PacketHandler<T> for DualConnectionsDevicePacketHandler
where
    T: Has<DualConnections> + Send + Sync,
{
    async fn handle_packet(
        &self,
        state: &watch::Sender<T>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: packet::inbound::DualConnectionsDevicePacket = packet.try_to_packet()?;
        state.send_modify(|state| {
            let dual_connections = state.get_mut();
            modify_state(dual_connections, packet);
        });
        Ok(())
    }
}

#[inline(never)]
fn modify_state(
    dual_connections: &mut DualConnections,
    packet: packet::inbound::DualConnectionsDevicePacket,
) {
    tracing::debug!(
        "got dual connections devices packet {}/{}",
        packet.current_packet_index,
        packet.total_packets
    );
    if packet.current_packet_index == 1 {
        dual_connections.devices.clear();
    }
    dual_connections.devices.extend(packet.devices);
}

#[cfg(test)]
mod tests {
    use macaddr::MacAddr6;
    use openscq30_lib_macros::Has;

    use crate::devices::soundcore::common::{
        packet::outbound::ToPacket, structures::DualConnectionsDevice,
    };

    use super::*;

    #[derive(Has)]
    struct TestState {
        dual_connections: DualConnections,
    }

    #[tokio::test(start_paused = true)]
    async fn devices_in_separate_packets() {
        let handler = DualConnectionsDevicePacketHandler;
        let (state_sender, state_receiver) = watch::channel(TestState {
            dual_connections: DualConnections {
                is_enabled: true,
                devices: Vec::new(),
            },
        });

        let devices = vec![
            DualConnectionsDevice {
                is_connected: true,
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 0),
                name: "First Device".to_string(),
            },
            DualConnectionsDevice {
                is_connected: false,
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 1),
                name: "Second Device".to_string(),
            },
            DualConnectionsDevice {
                is_connected: true,
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 2),
                name: "Third Device".to_string(),
            },
        ];

        for (i, device) in devices.iter().cloned().enumerate() {
            handler
                .handle_packet(
                    &state_sender,
                    &packet::inbound::DualConnectionsDevicePacket {
                        total_packets: devices.len() as u8,
                        current_packet_index: (i + 1) as u8,
                        devices: vec![device],
                    }
                    .to_packet(),
                )
                .await
                .expect(&format!("handle packet {i}"));
        }

        let state = state_receiver.borrow();
        assert_eq!(
            state.dual_connections,
            DualConnections {
                is_enabled: true,
                devices: devices.into_iter().collect(),
            }
        )
    }

    #[tokio::test(start_paused = true)]
    async fn devices_in_one_packet() {
        let handler = DualConnectionsDevicePacketHandler;
        let (state_sender, state_receiver) = watch::channel(TestState {
            dual_connections: DualConnections {
                is_enabled: true,
                devices: Vec::new(),
            },
        });

        let devices = vec![
            DualConnectionsDevice {
                is_connected: true,
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 0),
                name: "First Device".to_string(),
            },
            DualConnectionsDevice {
                is_connected: false,
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 1),
                name: "Second Device".to_string(),
            },
            DualConnectionsDevice {
                is_connected: true,
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 2),
                name: "Third Device".to_string(),
            },
        ];

        handler
            .handle_packet(
                &state_sender,
                &packet::inbound::DualConnectionsDevicePacket {
                    total_packets: 1,
                    current_packet_index: 1,
                    devices: devices.to_owned(),
                }
                .to_packet(),
            )
            .await
            .unwrap();

        let state = state_receiver.borrow();
        assert_eq!(
            state.dual_connections,
            DualConnections {
                is_enabled: true,
                devices: devices.into_iter().collect(),
            }
        )
    }
}

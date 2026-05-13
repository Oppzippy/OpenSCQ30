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
        state.send_if_modified(|state| {
            let dual_connections = state.get_mut();

            if dual_connections
                .devices
                .get(packet.index as usize)
                .and_then(|maybe_device| maybe_device.as_ref())
                .is_none_or(|device| *device != packet.device)
                || dual_connections.devices.len() != packet.total_devices as usize
            {
                let devices = &mut dual_connections.devices;

                devices.truncate(packet.total_devices as usize);
                while devices.len() < packet.total_devices as usize {
                    devices.push(None);
                }

                if let Some(index) = packet.index.checked_sub(1) {
                    tracing::trace!(
                        "updating dual connections device {}/{}",
                        packet.index,
                        packet.total_devices
                    );
                    devices[index as usize] = Some(packet.device);
                } else {
                    tracing::error!(
                        "dual connections device index should start from 1, but got {} ({} total)",
                        packet.index,
                        packet.total_devices,
                    );
                }
                true
            } else {
                tracing::trace!("got dual connections packet but no change");
                false
            }
        });
        Ok(())
    }
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
    async fn devices_in_order() {
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
                        total_devices: devices.len() as u8,
                        index: (i + 1) as u8,
                        device,
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
                devices: devices.into_iter().map(Some).collect(),
            }
        )
    }

    #[tokio::test(start_paused = true)]
    async fn devices_out_of_order() {
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

        // iterate in reverse order for out of order devices
        for (i, device) in devices.iter().cloned().enumerate().rev() {
            handler
                .handle_packet(
                    &state_sender,
                    &packet::inbound::DualConnectionsDevicePacket {
                        total_devices: devices.len() as u8,
                        index: (i + 1) as u8,
                        device,
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
                devices: devices.into_iter().map(Some).collect(),
            }
        )
    }

    #[tokio::test(start_paused = true)]
    async fn missing_devices() {
        let handler = DualConnectionsDevicePacketHandler;
        let (state_sender, state_receiver) = watch::channel(TestState {
            dual_connections: DualConnections {
                is_enabled: true,
                devices: Vec::new(),
            },
        });

        let devices = vec![
            Some(DualConnectionsDevice {
                is_connected: true,
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 0),
                name: "First Device".to_string(),
            }),
            None,
            Some(DualConnectionsDevice {
                is_connected: true,
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, 2),
                name: "Third Device".to_string(),
            }),
        ];

        // iterate in reverse order for out of order devices
        for (i, maybe_device) in devices.iter().cloned().enumerate() {
            if let Some(device) = maybe_device {
                handler
                    .handle_packet(
                        &state_sender,
                        &packet::inbound::DualConnectionsDevicePacket {
                            total_devices: devices.len() as u8,
                            index: (i + 1) as u8,
                            device,
                        }
                        .to_packet(),
                    )
                    .await
                    .expect(&format!("handle packet {i}"));
            }
        }

        let state = state_receiver.borrow();
        assert_eq!(
            state.dual_connections,
            DualConnections {
                is_enabled: true,
                devices,
            }
        )
    }

    #[tokio::test(start_paused = true)]
    async fn decrease_total_devices() {
        let handler = DualConnectionsDevicePacketHandler;
        let (state_sender, state_receiver) = watch::channel(TestState {
            dual_connections: DualConnections {
                is_enabled: true,
                devices: Vec::new(),
            },
        });

        let mut devices = vec![
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
                        total_devices: devices.len() as u8,
                        index: (i + 1) as u8,
                        device,
                    }
                    .to_packet(),
                )
                .await
                .expect(&format!("handle packet {i}"));
        }

        devices.pop();

        for (i, device) in devices.iter().cloned().enumerate() {
            handler
                .handle_packet(
                    &state_sender,
                    &packet::inbound::DualConnectionsDevicePacket {
                        total_devices: devices.len() as u8,
                        index: (i + 1) as u8,
                        device,
                    }
                    .to_packet(),
                )
                .await
                .expect(&format!("handle packet {i}"));
        }

        let state = state_receiver.borrow();
        assert_eq!(state.dual_connections.devices.len(), 2);
        assert_eq!(
            state.dual_connections,
            DualConnections {
                is_enabled: true,
                devices: devices.into_iter().map(Some).collect(),
            }
        )
    }
}

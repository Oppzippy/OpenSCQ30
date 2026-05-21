use async_trait::async_trait;
use macaddr::MacAddr6;
use openscq30_lib_has::MaybeHas;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
        structures::DualConnections,
    },
};

pub struct DualConnectionsStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl DualConnectionsStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<StateT> StateModifier<StateT> for DualConnectionsStateModifier
where
    StateT: MaybeHas<DualConnections> + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
    ) -> device::Result<()> {
        set_enabled(&self.packet_io, state_sender, target_state).await?;
        set_connected_devices(&self.packet_io, state_sender, target_state).await?;
        Ok(())
    }
}

async fn set_enabled<StateT>(
    packet_io: &PacketIOController,
    state_sender: &watch::Sender<StateT>,
    target_state: &StateT,
) -> device::Result<()>
where
    StateT: MaybeHas<DualConnections> + Send + Sync,
{
    let Some(target) = target_state.maybe_get() else {
        return Ok(());
    };
    let current_is_enabled = {
        let current = state_sender.borrow();
        let Some(dual_connections) = current.maybe_get() else {
            return Ok(());
        };
        dual_connections.is_enabled
    };

    if current_is_enabled != target.is_enabled {
        packet_io
            .send_with_response(&packet::outbound::set_dual_connections_enabled(
                target.is_enabled,
            ))
            .await?;
        state_sender.send_modify(|state| {
            if let Some(dual_connections) = state.maybe_get_mut() {
                dual_connections.is_enabled = target.is_enabled;
            }
        });
    }
    Ok(())
}

async fn set_connected_devices<StateT>(
    packet_io: &PacketIOController,
    state_sender: &watch::Sender<StateT>,
    target_state: &StateT,
) -> device::Result<()>
where
    StateT: MaybeHas<DualConnections> + Send + Sync,
{
    let Some(target) = target_state.maybe_get() else {
        return Ok(());
    };
    let target_devices = target.devices.iter().cloned().collect::<Vec<_>>();

    let devices = {
        let current = state_sender.borrow();
        let Some(dual_connections) = current.maybe_get() else {
            return Ok(());
        };

        assert!(
            target
                .devices
                .iter()
                .filter(|device| device.is_connected)
                .count()
                <= 2,
            "connecting to more than 2 devices is not possible: {:?}",
            target.devices,
        );

        dual_connections
            .devices
            .iter()
            // avoid cloning name strings
            .map(|device| DeviceWithoutName {
                is_connected: device.is_connected,
                mac_address: device.mac_address,
            })
            .collect::<Vec<DeviceWithoutName>>()
    };

    for device in &devices {
        if let Some(target_device) = target_devices
            .iter()
            .find(|d| d.mac_address == device.mac_address)
            && device.is_connected
            && !target_device.is_connected
        {
            // TODO acquire a lock so that multiple disconnections can't be going on at the same time
            // The lock should then be released by watching state sender with a timeout for when the device
            // becomes not connected.
            packet_io
                .send_without_response(&packet::outbound::dual_connections_disconnect(
                    device.mac_address,
                ))
                .await?;
        }
    }

    for device in &devices {
        if let Some(target_device) = target_devices
            .iter()
            .find(|d| d.mac_address == device.mac_address)
            && !device.is_connected
            && target_device.is_connected
        {
            // TODO acquire a lock so that multiple connections can't be going on at the same time
            // The lock should then be released by watching state sender with a timeout for when the device
            // becomes not connected.
            packet_io
                .send_without_response(&packet::outbound::dual_connections_connect(
                    device.mac_address,
                ))
                .await?;
        }
    }

    Ok(())
}

struct DeviceWithoutName {
    is_connected: bool,
    mac_address: MacAddr6,
}

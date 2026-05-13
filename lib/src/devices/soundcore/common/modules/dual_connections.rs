use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString};

use crate::{
    connection::RfcommConnection,
    device,
    devices::soundcore::common::{
        modules::ModuleCollection,
        packet::{self, PacketIOController, inbound::TryToPacket},
        structures::{DualConnections, DualConnectionsDevice},
    },
    macros::enum_subset,
    settings::{CategoryId, SettingId},
};

mod packet_handler;
mod setting_handler;
mod state_modifier;

enum_subset! {
    SettingId,
    #[derive(EnumIter, EnumString)]
    enum DualConnectionsSetting {
        DualConnections,
        DualConnectionsDevices,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<DualConnections> + Clone + Send + Sync,
{
    pub fn add_dual_connections<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.packet_handlers.set_handler(
            packet_handler::DualConnectionsDevicePacketHandler::COMMAND,
            Box::new(packet_handler::DualConnectionsDevicePacketHandler),
        );
        self.setting_manager.add_handler(
            CategoryId::DualConnections,
            setting_handler::DualConnectionsSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(state_modifier::DualConnectionsStateModifier::new(
                packet_io,
            )));
    }
}

pub async fn take_dual_connection_devices<ConnectionT>(
    packet_io: &PacketIOController<ConnectionT>,
) -> device::Result<Vec<Option<DualConnectionsDevice>>>
where
    ConnectionT: RfcommConnection + Send,
{
    // allow receiving packets out of order. this shouldn't happen, but just to be safe in case a device I'm not aware of does this.
    let mut devices: Vec<Option<DualConnectionsDevice>> = Vec::new();
    packet_io
        .send_with_multi_response(
            &packet::outbound::request_dual_connections_devices(),
            |packet| {
                let packet: packet::inbound::DualConnectionsDevicePacket = match packet
                    .try_to_packet()
                {
                    Ok(packet) => packet,
                    Err(err) => {
                        tracing::warn!("failed to parse dual connection device packet: {err:?}");
                        return false;
                    }
                };

                // In case the length somehow changes during the response
                devices.truncate(usize::from(packet.total_devices));
                while devices.len() < usize::from(packet.total_devices) {
                    devices.push(None);
                }

                // devices sends indices starting from 1, but our vec starts from 0
                match packet.index.checked_sub(1) {
                    Some(index) => {
                        devices[usize::from(index)] = Some(packet.device);
                    }
                    None => {
                        // If for some reason there is an index 0, we will ignore it and proceed as normal. This may
                        // lead to the last index never becoming Some if it's because all indices are offset by -1, but
                        // that's fine. We can just let send_with_multi_response time out.
                        tracing::error!(
                            "device index should start from 1, got {} of {} total",
                            packet.index,
                            packet.total_devices,
                        );
                    }
                };

                // devices is already the max length but padded with None, so we don't need to check the length
                devices.iter().any(|device| device.is_none())
            },
            20,
        )
        .await?;

    Ok(devices)
}

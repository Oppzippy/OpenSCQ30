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
                devices.truncate(packet.total_devices.into());
                while devices.len() < packet.total_devices.into() {
                    devices.push(None);
                }

                let index = packet
                    .index
                    .checked_sub(1)
                    .expect("device index should start from 1")
                    as usize;
                devices[index] = Some(packet.device);

                devices.len() < packet.total_devices.into()
                    || devices.iter().any(|device| device.is_none())
            },
            20,
        )
        .await?;

    Ok(devices)
}

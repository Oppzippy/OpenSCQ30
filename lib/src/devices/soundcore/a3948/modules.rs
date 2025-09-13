use openscq30_lib_has::Has;

use crate::{
    connection::RfcommConnection,
    devices::soundcore::{
        a3948,
        common::{
            device::SoundcoreDeviceBuilder, packet::inbound::InboundPacket, structures::TwsStatus,
        },
    },
};

mod button_configuration;

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateUpdatePacketType: InboundPacket + Into<StateType>,
    StateType: Has<a3948::structures::MultiButtonConfiguration>
        + Has<TwsStatus>
        + Send
        + Sync
        + Clone
        + 'static,
{
    pub fn a3948_button_configuration(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3948_button_configuration(packet_io_controller);
    }
}

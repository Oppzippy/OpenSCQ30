use openscq30_lib_has::Has;

use crate::{
    api::connection::RfcommConnection,
    devices::soundcore::common::{device::SoundcoreDeviceBuilder, packet::inbound::InboundPacket},
};

use super::structures::A3936SoundModes;

mod sound_modes;

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateUpdatePacketType: InboundPacket + Into<StateType>,
    StateType: Has<A3936SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn a3936_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3936_sound_modes(packet_io_controller);
    }
}

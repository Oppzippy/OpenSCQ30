use openscq30_lib_has::Has;

use crate::{
    api::connection::RfcommConnection,
    devices::soundcore::common::{device::SoundcoreDeviceBuilder, packet::inbound::FromPacketBody},
};

use super::structures::SoundModes;

mod sound_modes;

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateUpdatePacketType: FromPacketBody + Into<StateType>,
    StateType: Has<SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn a3959_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3959_sound_modes(packet_io_controller);
    }
}

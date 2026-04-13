use openscq30_lib_has::Has;

use crate::{
    connection::RfcommConnection,
    devices::soundcore::{a3952, common::device::SoundcoreDeviceBuilder},
};

mod sound_modes;

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<a3952::structures::SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn a3952_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3952_sound_modes(packet_io_controller);
    }
}

use openscq30_lib_has::Has;

use crate::{
    api::connection::RfcommConnection, devices::soundcore::common::device::SoundcoreDeviceBuilder,
};

use super::structures::SoundModes;

mod sound_modes;

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn a3959_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3959_sound_modes(packet_io_controller);
    }
}

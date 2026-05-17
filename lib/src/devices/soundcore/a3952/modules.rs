use openscq30_lib_has::Has;

use crate::devices::soundcore::{a3952, common::device::SoundcoreDeviceBuilder};

mod sound_modes;

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<a3952::structures::SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn a3952_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3952_sound_modes(packet_io_controller);
    }
}

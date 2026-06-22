use openscq30_lib_has::Has;

use crate::devices::soundcore::{a3957::state::A3957State, common::device::SoundcoreDeviceBuilder};

use super::structures::SoundModes;

mod flag;
mod sound_modes;

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn a3957_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3957_sound_modes(packet_io_controller);
    }
}

impl SoundcoreDeviceBuilder<A3957State> {
    pub fn a3957_gaming_mode(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3957_gaming_mode(packet_io_controller);
    }
}

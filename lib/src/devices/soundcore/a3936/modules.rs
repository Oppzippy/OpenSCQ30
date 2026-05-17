use openscq30_lib_has::Has;

use crate::devices::soundcore::common::device::SoundcoreDeviceBuilder;

use super::structures::A3936SoundModes;

mod sound_modes;

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<A3936SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn a3936_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3936_sound_modes(packet_io_controller);
    }
}

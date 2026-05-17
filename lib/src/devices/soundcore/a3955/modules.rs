use openscq30_lib_has::Has;

use crate::devices::soundcore::{
    a3955::structures::{AncPersonalizedToEarCanal, ImmersiveExperience},
    common::device::SoundcoreDeviceBuilder,
};

use super::structures::SoundModes;

mod immersive_experience;
mod sound_modes;

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<SoundModes> + Has<AncPersonalizedToEarCanal> + Send + Sync + Clone + 'static,
{
    pub fn a3955_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3955_sound_modes(packet_io_controller);
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<ImmersiveExperience> + Send + Sync + Clone + 'static,
{
    pub fn a3955_immersive_experience(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3955_immersive_experience(packet_io_controller);
    }
}

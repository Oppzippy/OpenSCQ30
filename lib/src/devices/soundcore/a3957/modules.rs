use openscq30_lib_has::Has;

use crate::{
    api::connection::RfcommConnection,
    devices::soundcore::common::device::SoundcoreDeviceBuilder,
};

use super::structures::{AncPersonalizedToEarCanal, ImmersiveExperience, SoundModes};

mod immersive_experience;
mod sound_modes;

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<SoundModes> + Has<AncPersonalizedToEarCanal> + Send + Sync + Clone + 'static,
{
    pub fn a3957_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3957_sound_modes(packet_io_controller);
    }
}

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<ImmersiveExperience> + Send + Sync + Clone + 'static,
{
    pub fn a3957_immersive_experience(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3957_immersive_experience(packet_io_controller);
    }
}

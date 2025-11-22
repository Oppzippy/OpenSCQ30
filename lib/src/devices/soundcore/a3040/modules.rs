use openscq30_lib_has::Has;

use crate::{
    connection::RfcommConnection,
    devices::soundcore::{
        a3040,
        common::{self, device::SoundcoreDeviceBuilder},
    },
};

mod button_configuration;
mod equalizer;
mod sound_modes;

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<a3040::structures::SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn a3040_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3040_sound_modes(packet_io_controller);
    }
}

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<common::structures::CommonEqualizerConfiguration<1, 10>>
        + Has<common::structures::CustomHearId<2, 10>>
        + Send
        + Sync
        + Clone
        + 'static,
{
    pub async fn a3040_equalizer(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();
        self.module_collection()
            .add_a3040_equalizer(packet_io_controller, database, device_model, change_notify)
            .await;
    }
}

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<a3040::structures::ButtonConfiguration> + Send + Sync + Clone + 'static,
{
    pub fn a3040_button_configuration(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3040_button_configuration(packet_io_controller);
    }
}

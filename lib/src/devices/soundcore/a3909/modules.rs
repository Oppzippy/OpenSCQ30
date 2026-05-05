use openscq30_lib_has::Has;

use crate::{
    connection::RfcommConnection,
    devices::soundcore::{a3909, common::device::SoundcoreDeviceBuilder},
};

pub mod equalizer;

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<a3909::structures::EqualizerConfiguration> + Clone + Send + Sync + 'static,
{
    pub async fn a3909_equalizer(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();
        self.module_collection()
            .add_a3909_equalizer(packet_io, database, device_model, change_notify)
            .await;
    }
}

use openscq30_lib_has::Has;

use crate::{
    connection::RfcommConnection,
    devices::soundcore::{
        a3116,
        common::{device::SoundcoreDeviceBuilder, structures::EqualizerConfiguration},
    },
};

mod auto_power_off;
mod equalizer;
mod power_off;
mod volume;

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<a3116::structures::AutoPowerOffDuration> + Send + Sync + Clone + 'static,
{
    pub fn a3116_auto_power_off(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3116_auto_power_off(packet_io_controller);
    }
}

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<a3116::structures::Volume> + Send + Sync + Clone + 'static,
{
    pub fn a3116_volume(&mut self, max_volume: u8) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3116_volume(packet_io_controller, max_volume);
    }
}

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<EqualizerConfiguration<1, 9, -6, 6, 0>> + Clone + Send + Sync + 'static,
{
    pub async fn a3116_equalizer(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();
        self.module_collection()
            .add_a3116_equalizer(packet_io, database, device_model, change_notify)
            .await;
    }
}

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<a3116::structures::PowerOffPending> + Clone + Send + Sync + 'static,
{
    pub fn a3116_power_off(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_a3116_power_off(packet_io);
    }
}

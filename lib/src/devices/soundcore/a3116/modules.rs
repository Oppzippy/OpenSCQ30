use openscq30_lib_has::Has;

use crate::{
    connection::RfcommConnection,
    devices::soundcore::{
        a3116,
        common::{device::SoundcoreDeviceBuilder, packet::inbound::FromPacketBody},
    },
};

mod auto_power_off;
mod equalizer;
mod volume;

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateUpdatePacketType: FromPacketBody + Into<StateType>,
    StateType: Has<a3116::structures::AutoPowerOffDuration> + Send + Sync + Clone + 'static,
{
    pub fn a3116_auto_power_off(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3116_auto_power_off(packet_io_controller);
    }
}

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateUpdatePacketType: FromPacketBody + Into<StateType>,
    StateType: Has<a3116::structures::Volume> + Send + Sync + Clone + 'static,
{
    pub fn a3116_volume(&mut self, max_volume: u8) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3116_volume(packet_io_controller, max_volume);
    }
}

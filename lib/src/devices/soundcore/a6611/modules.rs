use openscq30_lib_has::Has;

use crate::devices::soundcore::{
    a6611::structures::{BluetoothMode, FindDevice},
    common::device::SoundcoreDeviceBuilder,
};

mod find_device;
mod sleep_mode;

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<FindDevice> + Has<BluetoothMode> + Clone + Send + Sync + 'static,
{
    pub fn a6611_find_device(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_a6611_find_device(packet_io);
    }

    pub fn a6611_sleep_mode(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_a6611_sleep_mode(packet_io);
    }
}

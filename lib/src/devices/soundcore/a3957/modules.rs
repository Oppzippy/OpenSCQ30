use openscq30_lib_has::Has;

use crate::{
    api::connection::RfcommConnection,
    devices::soundcore::common::{
        device::SoundcoreDeviceBuilder,
        structures::{CaseBatteryLevel, DualBattery},
    },
};

use super::structures::{AncPersonalizedToEarCanal, SoundModes};

mod case_battery_level;
mod dual_battery;
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
    StateType: Has<DualBattery> + Send + Sync + Clone + 'static,
{
    pub fn a3957_dual_battery(&mut self, max_level: u8) {
        self.module_collection().add_a3957_dual_battery(max_level);
    }
}

impl<ConnectionType, StateType> SoundcoreDeviceBuilder<ConnectionType, StateType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Has<CaseBatteryLevel> + Send + Sync + Clone + 'static,
{
    pub fn a3957_case_battery_level(&mut self, max_level: u8) {
        self.module_collection()
            .add_a3957_case_battery_level(max_level);
    }
}

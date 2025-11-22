use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3116,
    common::structures::{EqualizerConfiguration, FirmwareVersion, SerialNumber, SingleBattery},
};

use super::packets::inbound::A3116StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3116State {
    battery: SingleBattery,
    volume: a3116::structures::Volume,
    auto_power_off_duration: a3116::structures::AutoPowerOffDuration,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: EqualizerConfiguration<1, 9, -6, 6, 0>,
}

impl From<A3116StateUpdatePacket> for A3116State {
    fn from(value: A3116StateUpdatePacket) -> Self {
        Self {
            battery: value.battery,
            volume: value.volume,
            auto_power_off_duration: value.auto_power_off_duration,
            firmware_version: value.firmware_version,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
        }
    }
}

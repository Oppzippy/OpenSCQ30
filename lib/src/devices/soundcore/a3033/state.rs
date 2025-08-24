use openscq30_lib_macros::Has;

use crate::devices::soundcore::standard::structures::{
    EqualizerConfiguration, FirmwareVersion, SerialNumber, SingleBattery,
};

use super::packets::A3033StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3033State {
    battery: SingleBattery,
    equalizer_configuration: EqualizerConfiguration<1, 8>,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    #[has(skip)]
    wear_detection: bool,
}

impl From<A3033StateUpdatePacket> for A3033State {
    fn from(value: A3033StateUpdatePacket) -> Self {
        Self {
            battery: value.battery,
            equalizer_configuration: value.equalizer_configuration,
            firmware_version: value.firmware_version,
            serial_number: value.serial_number,
            wear_detection: value.wear_detection,
        }
    }
}

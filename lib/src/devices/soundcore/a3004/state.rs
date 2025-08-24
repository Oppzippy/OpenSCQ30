use openscq30_lib_macros::Has;

use crate::devices::soundcore::standard::structures::{
    EqualizerConfiguration, FirmwareVersion, SerialNumber, SingleBattery, SoundModes,
};

use super::packets::A3004StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3004State {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration<1, 10>,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
}

impl From<A3004StateUpdatePacket> for A3004State {
    fn from(value: A3004StateUpdatePacket) -> Self {
        Self {
            battery: value.battery,
            equalizer_configuration: value.equalizer_configuration,
            sound_modes: value.sound_modes,
            firmware_version: value.firmware_version,
            serial_number: value.serial_number,
        }
    }
}

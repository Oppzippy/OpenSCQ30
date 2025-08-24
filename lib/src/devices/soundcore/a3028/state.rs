use openscq30_lib_macros::Has;

use crate::devices::soundcore::standard::structures::{
    AgeRange, AutoPowerOff, BasicHearId, EqualizerConfiguration, FirmwareVersion, Gender,
    SerialNumber, SingleBattery, SoundModes,
};

use super::packets::A3028StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3028State {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration<1, 8>,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId<2, 8>,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    #[has(maybe)]
    pub auto_power_off: Option<AutoPowerOff>,
}

impl From<A3028StateUpdatePacket> for A3028State {
    fn from(value: A3028StateUpdatePacket) -> Self {
        Self {
            battery: value.battery,
            equalizer_configuration: value.equalizer_configuration,
            gender: value.gender,
            age_range: value.age_range,
            hear_id: value.hear_id,
            sound_modes: value.sound_modes,
            firmware_version: value.firmware_version,
            serial_number: value.serial_number,
            auto_power_off: value.extra_fields.map(|extras| extras.auto_power_off),
        }
    }
}

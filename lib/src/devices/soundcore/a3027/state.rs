use openscq30_lib_macros::Has;

use crate::devices::soundcore::standard::structures::{
    AgeRange, BasicHearId, EqualizerConfiguration, FirmwareVersion, Gender, SerialNumber,
    SingleBattery, SoundModes,
};

use super::packets::A3027StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3027State {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration<1, 8>,
    pub gender: Gender,
    pub age_range: AgeRange,
    // Two channels, but the second one is ignored
    pub hear_id: BasicHearId<2, 8>,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    #[has(skip)]
    pub wear_detection: bool,
    #[has(skip)]
    pub touch_func: Option<bool>,
}

impl From<A3027StateUpdatePacket> for A3027State {
    fn from(value: A3027StateUpdatePacket) -> Self {
        Self {
            battery: value.battery,
            equalizer_configuration: value.equalizer_configuration,
            gender: value.gender,
            age_range: value.age_range,
            hear_id: value.hear_id,
            sound_modes: value.sound_modes,
            firmware_version: value.firmware_version,
            serial_number: value.serial_number,
            wear_detection: value.wear_detection,
            touch_func: value.touch_func,
        }
    }
}

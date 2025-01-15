use crate::devices::standard::structures::{
    AgeRange, BasicHearId, EqualizerConfiguration, FirmwareVersion, Gender, SerialNumber,
    SingleBattery, SoundModes,
};

use super::packets::A3027StateUpdatePacket;

#[derive(Debug, Clone, PartialEq)]
pub struct A3027State {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub wear_detection: bool,
    pub touch_func: Option<bool>,
}

impl AsMut<SoundModes> for A3027State {
    fn as_mut(&mut self) -> &mut SoundModes {
        &mut self.sound_modes
    }
}
impl AsRef<SoundModes> for A3027State {
    fn as_ref(&self) -> &SoundModes {
        &self.sound_modes
    }
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

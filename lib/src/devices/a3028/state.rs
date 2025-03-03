use crate::devices::standard::structures::{
    AgeRange, BasicHearId, EqualizerConfiguration, FirmwareVersion, Gender, SerialNumber,
    SingleBattery, SoundModes,
};

use super::packets::{A3028StateUpdatePacket, ExtraFields};

#[derive(Debug, Clone, PartialEq)]
pub struct A3028State {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub extra_fields: Option<ExtraFields>,
}

impl AsRef<SoundModes> for A3028State {
    fn as_ref(&self) -> &SoundModes {
        &self.sound_modes
    }
}
impl AsMut<SoundModes> for A3028State {
    fn as_mut(&mut self) -> &mut SoundModes {
        &mut self.sound_modes
    }
}

impl AsRef<EqualizerConfiguration> for A3028State {
    fn as_ref(&self) -> &EqualizerConfiguration {
        &self.equalizer_configuration
    }
}
impl AsMut<EqualizerConfiguration> for A3028State {
    fn as_mut(&mut self) -> &mut EqualizerConfiguration {
        &mut self.equalizer_configuration
    }
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
            extra_fields: value.extra_fields,
        }
    }
}

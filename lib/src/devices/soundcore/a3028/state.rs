use crate::{
    devices::soundcore::standard::structures::{
        AgeRange, BasicHearId, EqualizerConfiguration, FirmwareVersion, Gender, SerialNumber,
        SingleBattery, SoundModes,
    },
    macros::impl_as_ref_for_field,
};

use super::packets::{A3028StateUpdatePacket, ExtraFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3028State {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration<1, 8>,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId<2, 8>,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub extra_fields: Option<ExtraFields>,
}

impl_as_ref_for_field!(
    struct A3028State {
        battery: SingleBattery,
        sound_modes: SoundModes,
        equalizer_configuration: EqualizerConfiguration<1, 8>,
        firmware_version: FirmwareVersion,
        serial_number: SerialNumber,
    }
);

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

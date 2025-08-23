use crate::{
    devices::soundcore::standard::structures::{
        AgeRange, AutoPowerOff, BasicHearId, EqualizerConfiguration, FirmwareVersion, Gender,
        SerialNumber, SingleBattery, SoundModes,
    },
    has::MaybeHas,
    macros::impl_as_ref_for_field,
};

use super::packets::A3028StateUpdatePacket;

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
    pub auto_power_off: Option<AutoPowerOff>,
}

impl MaybeHas<AutoPowerOff> for A3028State {
    fn maybe_get(&self) -> Option<&AutoPowerOff> {
        self.auto_power_off.as_ref()
    }

    fn maybe_get_mut(&mut self) -> Option<&mut AutoPowerOff> {
        self.auto_power_off.as_mut()
    }

    fn set_maybe(&mut self, value: Option<AutoPowerOff>) {
        self.auto_power_off = value;
    }
}

impl_as_ref_for_field!(
    struct A3028State {
        battery: SingleBattery,
        sound_modes: SoundModes,
        equalizer_configuration: EqualizerConfiguration<1, 8>,
        firmware_version: FirmwareVersion,
        serial_number: SerialNumber,
        auto_power_off: Option<AutoPowerOff>,
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
            auto_power_off: value.extra_fields.map(|extras| extras.auto_power_off),
        }
    }
}

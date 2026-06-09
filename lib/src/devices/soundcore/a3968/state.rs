use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3968,
    common::structures::{
        AgeRange, CaseBatteryLevel, CommonEqualizerConfiguration, CustomHearId, DualBattery,
        DualFirmwareVersion, Gender, SerialNumber, TwsStatus,
    },
};

use super::packets::inbound::A3968StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3968State {
    tws_status: TwsStatus,
    dual_battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    case_battery_level: CaseBatteryLevel,
    sound_modes: a3968::structures::SoundModes,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    hear_id: CustomHearId<2, 10>,
    age_range: AgeRange,
    gender: Gender,
}

impl From<A3968StateUpdatePacket> for A3968State {
    fn from(value: A3968StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            dual_battery: value.dual_battery,
            dual_firmware_version: value.dual_firmware_version,
            serial_number: value.serial_number,
            case_battery_level: value.case_battery_level,
            equalizer_configuration: value.equalizer_configuration,
            hear_id: value.hear_id,
            sound_modes: value.sound_modes,
            age_range: AgeRange::default(),
            gender: Gender::default(),
        }
    }
}

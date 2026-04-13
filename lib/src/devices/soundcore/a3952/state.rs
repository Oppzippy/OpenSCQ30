use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3952,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        structures::{
            AgeRange, AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel,
            CommonEqualizerConfiguration, CustomHearId, DualBattery, DualFirmwareVersion, Gender,
            SerialNumber, TouchTone, TwsStatus, WearingDetection, WearingTone,
            button_configuration::ButtonStatusCollection,
        },
    },
};

use super::packets::inbound::A3952StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3952State {
    tws_status: TwsStatus,
    battery: DualBattery,
    firmware: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    hear_id: CustomHearId<2, 10>,
    buttons: ButtonStatusCollection<6>,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3952::structures::SoundModes,
    touch_tone: TouchTone,
    wear_detection: WearingDetection,
    case_battery_level: CaseBatteryLevel,
    wearing_tone: WearingTone,
    auto_power_off: AutoPowerOff,
    gender: Gender,
    age_range: AgeRange,
    reset_pending: ResetButtonConfigurationPending,
}

impl From<A3952StateUpdatePacket> for A3952State {
    fn from(value: A3952StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            firmware: value.firmware_version,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
            hear_id: value.hear_id,
            buttons: value.buttons,
            ambient_sound_mode_cycle: value.ambient_sound_mode_cycle,
            sound_modes: value.sound_modes,
            touch_tone: value.touch_tone,
            wear_detection: value.wear_detection,
            case_battery_level: value.case_battery_level,
            wearing_tone: value.wearing_tone,
            auto_power_off: value.auto_power_off,
            age_range: value.age_range,
            gender: Gender::default(), // unknown
            reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

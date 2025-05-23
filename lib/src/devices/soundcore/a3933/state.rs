use crate::{
    devices::soundcore::standard::structures::{
        AgeRange, AmbientSoundModeCycle, BatteryLevel, CustomHearId, DualBattery,
        DualFirmwareVersion, EqualizerConfiguration, MultiButtonConfiguration, SerialNumber,
        SoundModes, TwsStatus,
    },
    macros::impl_as_ref_for_field,
};

use super::packets::inbound::A3933StateUpdatePacket;

#[derive(Debug, Clone, PartialEq)]
pub struct A3933State {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub age_range: AgeRange,
    pub hear_id: Option<CustomHearId<2, 10>>,
    pub button_configuration: MultiButtonConfiguration,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: SoundModes,
    pub touch_tone_switch: bool,
    pub wear_detection_switch: bool,
    pub game_mode_switch: bool,
    pub charging_case_battery_level: BatteryLevel,
    pub device_color: u8,
    pub wind_noise_detection: bool,
}

impl From<A3933StateUpdatePacket> for A3933State {
    fn from(value: A3933StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            dual_firmware_version: value.dual_firmware_version,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
            age_range: value.age_range,
            hear_id: value.hear_id,
            button_configuration: value.button_configuration,
            ambient_sound_mode_cycle: value.ambient_sound_mode_cycle,
            sound_modes: value.sound_modes,
            touch_tone_switch: value.touch_tone_switch,
            wear_detection_switch: value.wear_detection_switch,
            game_mode_switch: value.game_mode_switch,
            charging_case_battery_level: value.charging_case_battery_level,
            device_color: value.device_color,
            wind_noise_detection: value.wind_noise_detection,
        }
    }
}

impl_as_ref_for_field!(
    struct A3933State {
        tws_status: TwsStatus,
        battery: DualBattery,
        dual_firmware_version: DualFirmwareVersion,
        serial_number: SerialNumber,
        equalizer_configuration: EqualizerConfiguration<2, 10>,
        age_range: AgeRange,
        hear_id: Option<CustomHearId<2, 10>>,
        button_configuration: MultiButtonConfiguration,
        ambient_sound_mode_cycle: AmbientSoundModeCycle,
        sound_modes: SoundModes,
    }
);

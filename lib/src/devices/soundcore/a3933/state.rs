use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    modules::reset_button_configuration::ResetButtonConfigurationPending,
    structures::{
        AgeRange, AmbientSoundModeCycle, CaseBatteryLevel, CustomHearId, DualBattery,
        DualFirmwareVersion, EqualizerConfiguration, SerialNumber, SoundModes, TouchTone,
        TwsStatus, button_configuration::ButtonStatusCollection,
    },
};

use super::packets::inbound::A3933StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3933State {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub age_range: AgeRange,
    pub hear_id: Option<CustomHearId<2, 10>>,
    pub button_configuration: ButtonStatusCollection<6>,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: SoundModes,
    pub case_battery_level: CaseBatteryLevel,
    pub touch_tone_switch: TouchTone,
    #[has(skip)]
    pub wear_detection_switch: bool,
    #[has(skip)]
    pub game_mode_switch: bool,
    #[has(skip)]
    pub device_color: u8,
    #[has(skip)]
    pub wind_noise_detection: bool,
    button_reset_pending: ResetButtonConfigurationPending,
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
            touch_tone_switch: value.touch_tone,
            wear_detection_switch: value.wear_detection_switch,
            game_mode_switch: value.game_mode_switch,
            case_battery_level: value.case_battery_level,
            device_color: value.device_color,
            wind_noise_detection: value.wind_noise_detection,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

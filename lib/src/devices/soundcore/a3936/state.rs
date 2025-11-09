use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    modules::reset_button_configuration::ResetButtonConfigurationPending,
    structures::{
        AgeRange, AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel, CustomHearId, DualBattery,
        DualFirmwareVersion, EqualizerConfiguration, GamingMode, Gender, SerialNumber, TouchTone,
        TwsStatus, button_configuration::ButtonStatusCollection,
    },
};

use super::{packets::A3936StateUpdatePacket, structures::A3936SoundModes};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3936State {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub age_range: AgeRange,
    pub custom_hear_id: CustomHearId<2, 10>,
    pub sound_modes: A3936SoundModes,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub button_configuration: ButtonStatusCollection<6>,
    pub case_battery_level: CaseBatteryLevel,
    pub auto_power_off: AutoPowerOff,
    pub gender: Gender,
    pub touch_tone: TouchTone,
    pub gaming_mode: GamingMode,
    #[has(skip)]
    pub color: u8,
    #[has(skip)]
    pub ldac: bool,
    #[has(skip)]
    pub supports_two_cnn_switch: bool,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl From<A3936StateUpdatePacket> for A3936State {
    fn from(value: A3936StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            dual_firmware_version: value.dual_firmware_version,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
            age_range: value.age_range,
            custom_hear_id: value.custom_hear_id,
            sound_modes: value.sound_modes,
            ambient_sound_mode_cycle: value.ambient_sound_mode_cycle,
            button_configuration: value.button_configuration,
            touch_tone: value.touch_tone,
            case_battery_level: value.case_battery_level,
            color: value.color,
            ldac: value.ldac,
            supports_two_cnn_switch: value.supports_two_cnn_switch,
            auto_power_off: value.auto_power_off,
            gaming_mode: value.gaming_mode,
            gender: Gender::default(),
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

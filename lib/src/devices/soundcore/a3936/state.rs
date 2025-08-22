use crate::{
    devices::soundcore::standard::structures::{
        AgeRange, AmbientSoundModeCycle, AutoPowerOff, BatteryLevel, CustomHearId, DualBattery,
        DualFirmwareVersion, EqualizerConfiguration, Gender, SerialNumber, TwsStatus,
    },
    macros::impl_as_ref_for_field,
};

use super::{
    packets::A3936StateUpdatePacket,
    structures::{A3936InternalMultiButtonConfiguration, A3936SoundModes},
};

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub button_configuration: A3936InternalMultiButtonConfiguration,
    pub touch_tone: bool,
    pub charging_case_battery: BatteryLevel,
    pub color: u8,
    pub ldac: bool,
    pub supports_two_cnn_switch: bool,
    pub auto_power_off: AutoPowerOff,
    pub game_mode_switch: bool,
}

impl_as_ref_for_field!(
    struct A3936State {
        tws_status: TwsStatus,
        battery: DualBattery,
        dual_firmware_version: DualFirmwareVersion,
        serial_number: SerialNumber,
        equalizer_configuration: EqualizerConfiguration<2, 10>,
        age_range: AgeRange,
        custom_hear_id: CustomHearId<2, 10>,
        sound_modes: A3936SoundModes,
        ambient_sound_mode_cycle: AmbientSoundModeCycle,
        button_configuration: A3936InternalMultiButtonConfiguration,
        auto_power_off: AutoPowerOff,
    }
);

impl AsRef<Gender> for A3936State {
    fn as_ref(&self) -> &Gender {
        &Gender(0)
    }
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
            charging_case_battery: value.charging_case_battery,
            color: value.color,
            ldac: value.ldac,
            supports_two_cnn_switch: value.supports_two_cnn_switch,
            auto_power_off: value.auto_power_off,
            game_mode_switch: value.game_mode_switch,
        }
    }
}

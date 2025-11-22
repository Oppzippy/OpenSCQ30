use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3040::{self, packets::A3040StateUpdatePacket},
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, BatteryLevel, CommonEqualizerConfiguration,
            CustomHearId, FirmwareVersion, LimitHighVolume, SerialNumber,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default, Has)]
pub struct A3040State {
    battery_level: BatteryLevel,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    button_configuration: a3040::structures::ButtonConfiguration,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3040::structures::SoundModes,
    auto_power_off: AutoPowerOff,
    limit_high_volume: LimitHighVolume,
    hear_id: CustomHearId<2, 10>,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl From<A3040StateUpdatePacket> for A3040State {
    fn from(value: A3040StateUpdatePacket) -> Self {
        let A3040StateUpdatePacket {
            battery_level,
            firmware_version,
            serial_number,
            equalizer_configuration,
            button_configuration,
            ambient_sound_mode_cycle,
            sound_modes,
            auto_power_off,
            limit_high_volume,
            ambient_sound_mode_prompt_tone: _,
            battery_alert_prompt_tone: _,
            hear_id,
        } = value;

        Self {
            battery_level,
            firmware_version,
            serial_number,
            equalizer_configuration,
            button_configuration,
            ambient_sound_mode_cycle,
            sound_modes,
            auto_power_off,
            limit_high_volume,
            hear_id,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

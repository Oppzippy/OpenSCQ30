use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3035::{self, packets::inbound::A3035StateUpdatePacket},
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        structures::{
            AmbientSoundModeCycle, AutoPlayPause, AutoPowerOff, BatteryLevel,
            CommonEqualizerConfiguration, CustomHearId, FirmwareVersion, LimitHighVolume,
            SerialNumber,
        },
    },
};

#[derive(Has, Clone)]
pub struct A3035State {
    battery_level: BatteryLevel,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    hear_id: CustomHearId<1, 10>,
    button_configuration: a3035::structures::ButtonConfiguration,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3035::structures::SoundModes,
    auto_play_pause: AutoPlayPause,
    auto_power_off: AutoPowerOff,
    limit_high_volume: LimitHighVolume,
    ambient_sound_mode_voice_prompt: a3035::structures::AmbientSoundModeVoicePrompt,
    battery_alert: a3035::structures::BatteryAlert,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl From<A3035StateUpdatePacket> for A3035State {
    fn from(value: A3035StateUpdatePacket) -> Self {
        Self {
            battery_level: value.battery_level,
            firmware_version: value.firmware_version,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
            hear_id: value.hear_id,
            button_configuration: value.button_configuration,
            ambient_sound_mode_cycle: value.ambient_sound_mode_cycle,
            sound_modes: value.sound_modes,
            auto_play_pause: value.auto_play_pause,
            auto_power_off: value.auto_power_off,
            limit_high_volume: value.limit_high_volume,
            ambient_sound_mode_voice_prompt: value.ambient_sound_mode_voice_prompt,
            battery_alert: value.battery_alert,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

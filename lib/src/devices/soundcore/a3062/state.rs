use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3062,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, CommonEqualizerConfiguration, CustomHearId,
            FirmwareVersion, Ldac, LimitHighVolume, LowBatteryPrompt, SerialNumber, SingleBattery,
        },
    },
};

use super::packets::inbound::A3062StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3062State {
    battery: SingleBattery,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    hear_id: CustomHearId<1, 10>,
    button_configuration: a3062::structures::ButtonConfiguration,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3062::structures::SoundModes,
    low_battery_prompt: LowBatteryPrompt,
    auto_power_off: AutoPowerOff,
    limit_high_volume: LimitHighVolume,
    side_tone: a3062::structures::SideTone,
    ambient_sound_mode_voice_prompt: a3062::structures::AmbientSoundModeVoicePrompt,
    dolby_audio: a3062::structures::DolbyAudio,
    ldac: Ldac,
    button_configuration_reset: ResetButtonConfigurationPending,
}

impl From<A3062StateUpdatePacket> for A3062State {
    fn from(value: A3062StateUpdatePacket) -> Self {
        Self {
            battery: value.battery,
            firmware_version: value.firmware_version,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
            hear_id: value.hear_id,
            button_configuration: value.button_configuration,
            ambient_sound_mode_cycle: value.ambient_sound_mode_cycle,
            sound_modes: value.sound_modes,
            low_battery_prompt: value.low_battery_prompt,
            auto_power_off: value.auto_power_off,
            limit_high_volume: value.limit_high_volume,
            side_tone: value.side_tone,
            ambient_sound_mode_voice_prompt: value.ambient_sound_mode_voice_prompt,
            dolby_audio: value.dolby_audio,
            ldac: value.ldac,
            button_configuration_reset: ResetButtonConfigurationPending::default(),
        }
    }
}

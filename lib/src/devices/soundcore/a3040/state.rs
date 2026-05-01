use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3040::{self, packets::A3040StateUpdatePacket},
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, BatteryLevel, CommonEqualizerConfiguration,
            CustomHearId, FirmwareVersion, Ldac, LimitHighVolume, SerialNumber,
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
    voice_prompt: a3040::structures::VoicePrompt,
    low_battery_prompt: a3040::structures::LowBatteryPrompt,
    ldac: Ldac,
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
            voice_prompt,
            low_battery_prompt,
            hear_id,
            ldac,
            dual_connections: _,
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
            voice_prompt,
            low_battery_prompt,
            ldac,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

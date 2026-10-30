use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AgeRange, AmbientSoundModeCycle, AutoPlayPause, AutoPowerOff,
            CommonEqualizerConfiguration, CustomHearId, DualConnections, DualConnectionsDevice,
            FirmwareVersion, Ldac, LimitHighVolume, LowBatteryPrompt, SerialNumber, SingleBattery,
            SurroundSound,
        },
    },
    d1402,
};

use super::packets::inbound::D1402StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct D1402State {
    battery: SingleBattery,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    equalizer_unknown_byte: d1402::structures::EqualizerUnknownByte,
    age_range: AgeRange,
    hear_id: CustomHearId<1, 10>,
    button_configuration: d1402::structures::ButtonConfiguration,
    mfb_button_configuration: d1402::structures::MfbButtonConfiguration,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: d1402::structures::SoundModes,
    three_d_sound: SurroundSound,
    ambient_sound_mode_voice_prompt: d1402::structures::AmbientSoundModeVoicePrompt,
    low_battery_prompt: LowBatteryPrompt,
    ldac: Ldac,
    auto_power_off: AutoPowerOff,
    limit_high_volume: LimitHighVolume,
    auto_play_pause: AutoPlayPause,
    hey_anka_wake_word: d1402::structures::HeyAnkaWakeWord,
    dual_connections: DualConnections,
    button_configuration_reset: ResetButtonConfigurationPending,
}

impl D1402State {
    pub fn new(
        state_update_packet: D1402StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            battery: state_update_packet.battery,
            firmware_version: state_update_packet.firmware_version,
            serial_number: state_update_packet.serial_number,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            equalizer_unknown_byte: state_update_packet.equalizer_unknown_byte,
            age_range: state_update_packet.age_range,
            hear_id: state_update_packet.hear_id,
            button_configuration: state_update_packet.button_configuration,
            mfb_button_configuration: state_update_packet.mfb_button_configuration,
            ambient_sound_mode_cycle: state_update_packet.ambient_sound_mode_cycle,
            sound_modes: state_update_packet.sound_modes,
            three_d_sound: state_update_packet.three_d_sound,
            ambient_sound_mode_voice_prompt: state_update_packet.ambient_sound_mode_voice_prompt,
            low_battery_prompt: state_update_packet.low_battery_prompt,
            ldac: state_update_packet.ldac,
            auto_power_off: state_update_packet.auto_power_off,
            limit_high_volume: state_update_packet.limit_high_volume,
            auto_play_pause: state_update_packet.auto_play_pause,
            hey_anka_wake_word: state_update_packet.hey_anka_wake_word,
            dual_connections: DualConnections {
                is_enabled: state_update_packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            button_configuration_reset: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<D1402StateUpdatePacket> for D1402State {
    fn update(&mut self, partial: D1402StateUpdatePacket) {
        let D1402StateUpdatePacket {
            battery,
            firmware_version,
            serial_number,
            equalizer_configuration,
            equalizer_unknown_byte,
            age_range,
            hear_id,
            button_configuration,
            ambient_sound_mode_cycle,
            mfb_button_configuration,
            sound_modes,
            three_d_sound,
            ambient_sound_mode_voice_prompt,
            low_battery_prompt,
            ldac,
            dual_connections_enabled,
            auto_power_off,
            limit_high_volume,
            auto_play_pause,
            hey_anka_wake_word,
        } = partial;
        self.battery = battery;
        self.firmware_version = firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.equalizer_unknown_byte = equalizer_unknown_byte;
        self.age_range = age_range;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
        self.mfb_button_configuration = mfb_button_configuration;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.sound_modes = sound_modes;
        self.three_d_sound = three_d_sound;
        self.ambient_sound_mode_voice_prompt = ambient_sound_mode_voice_prompt;
        self.low_battery_prompt = low_battery_prompt;
        self.ldac = ldac;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.auto_power_off = auto_power_off;
        self.limit_high_volume = limit_high_volume;
        self.auto_play_pause = auto_play_pause;
        self.hey_anka_wake_word = hey_anka_wake_word;
    }
}

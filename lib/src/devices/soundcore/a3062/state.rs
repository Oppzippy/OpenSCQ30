use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3062,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, CommonEqualizerConfiguration, CustomHearId,
            DualConnections, DualConnectionsDevice, FirmwareVersion, Ldac, LimitHighVolume,
            LowBatteryPrompt, SerialNumber, SingleBattery,
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
    dual_connections: DualConnections,
    button_configuration_reset: ResetButtonConfigurationPending,
}

impl A3062State {
    pub fn new(
        state_update_packet: A3062StateUpdatePacket,
        dual_connections_devices: Vec<Option<DualConnectionsDevice>>,
    ) -> Self {
        Self {
            battery: state_update_packet.battery,
            firmware_version: state_update_packet.firmware_version,
            serial_number: state_update_packet.serial_number,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            hear_id: state_update_packet.hear_id,
            button_configuration: state_update_packet.button_configuration,
            ambient_sound_mode_cycle: state_update_packet.ambient_sound_mode_cycle,
            sound_modes: state_update_packet.sound_modes,
            low_battery_prompt: state_update_packet.low_battery_prompt,
            auto_power_off: state_update_packet.auto_power_off,
            limit_high_volume: state_update_packet.limit_high_volume,
            side_tone: state_update_packet.side_tone,
            ambient_sound_mode_voice_prompt: state_update_packet.ambient_sound_mode_voice_prompt,
            dolby_audio: state_update_packet.dolby_audio,
            ldac: state_update_packet.ldac,
            dual_connections: DualConnections {
                is_enabled: state_update_packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            button_configuration_reset: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<A3062StateUpdatePacket> for A3062State {
    fn update(&mut self, partial: A3062StateUpdatePacket) {
        let A3062StateUpdatePacket {
            battery,
            firmware_version,
            serial_number,
            equalizer_configuration,
            hear_id,
            button_configuration,
            ambient_sound_mode_cycle,
            sound_modes,
            low_battery_prompt,
            dolby_audio,
            ldac,
            dual_connections_enabled,
            auto_power_off,
            limit_high_volume,
            side_tone,
            ambient_sound_mode_voice_prompt,
        } = partial;
        self.battery = battery;
        self.firmware_version = firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.sound_modes = sound_modes;
        self.low_battery_prompt = low_battery_prompt;
        self.dolby_audio = dolby_audio;
        self.ldac = ldac;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.auto_power_off = auto_power_off;
        self.limit_high_volume = limit_high_volume;
        self.side_tone = side_tone;
        self.ambient_sound_mode_voice_prompt = ambient_sound_mode_voice_prompt;
    }
}

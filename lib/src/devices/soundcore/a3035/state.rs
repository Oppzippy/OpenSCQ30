use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3035::{self, packets::inbound::A3035StateUpdatePacket},
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AmbientSoundModeCycle, AutoPlayPause, AutoPowerOff, BatteryLevel,
            CommonEqualizerConfiguration, CustomHearId, DualConnections, DualConnectionsDevice,
            FirmwareVersion, Ldac, LimitHighVolume, SerialNumber,
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
    ldac: Ldac,
    dual_connections: DualConnections,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl A3035State {
    pub fn new(
        state_update_packet: A3035StateUpdatePacket,
        dual_connections_devices: Vec<Option<DualConnectionsDevice>>,
    ) -> Self {
        Self {
            battery_level: state_update_packet.battery_level,
            firmware_version: state_update_packet.firmware_version,
            serial_number: state_update_packet.serial_number,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            hear_id: state_update_packet.hear_id,
            button_configuration: state_update_packet.button_configuration,
            ambient_sound_mode_cycle: state_update_packet.ambient_sound_mode_cycle,
            sound_modes: state_update_packet.sound_modes,
            auto_play_pause: state_update_packet.auto_play_pause,
            auto_power_off: state_update_packet.auto_power_off,
            limit_high_volume: state_update_packet.limit_high_volume,
            ambient_sound_mode_voice_prompt: state_update_packet.ambient_sound_mode_voice_prompt,
            battery_alert: state_update_packet.battery_alert,
            ldac: state_update_packet.ldac,
            dual_connections: DualConnections {
                is_enabled: state_update_packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<A3035StateUpdatePacket> for A3035State {
    fn update(&mut self, partial: A3035StateUpdatePacket) {
        let A3035StateUpdatePacket {
            battery_level,
            firmware_version,
            serial_number,
            equalizer_configuration,
            hear_id,
            button_configuration,
            ambient_sound_mode_cycle,
            sound_modes,
            auto_play_pause,
            auto_power_off,
            limit_high_volume,
            ambient_sound_mode_voice_prompt,
            battery_alert,
            ldac,
            dual_connections_enabled,
        } = partial;
        self.battery_level = battery_level;
        self.firmware_version = firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.sound_modes = sound_modes;
        self.auto_play_pause = auto_play_pause;
        self.auto_power_off = auto_power_off;
        self.limit_high_volume = limit_high_volume;
        self.ambient_sound_mode_voice_prompt = ambient_sound_mode_voice_prompt;
        self.battery_alert = battery_alert;
        self.ldac = ldac;
        self.dual_connections.is_enabled = dual_connections_enabled;
    }
}

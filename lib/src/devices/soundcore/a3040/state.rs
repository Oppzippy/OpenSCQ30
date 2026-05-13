use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3040::{self, packets::A3040StateUpdatePacket},
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, BatteryLevel, CommonEqualizerConfiguration,
            CustomHearId, DualConnections, DualConnectionsDevice, FirmwareVersion, Ldac,
            LimitHighVolume, SerialNumber,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
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
    dual_connections: DualConnections,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl A3040State {
    pub fn new(
        state_update_packet: A3040StateUpdatePacket,
        dual_connections_devices: Vec<Option<DualConnectionsDevice>>,
    ) -> Self {
        Self {
            battery_level: state_update_packet.battery_level,
            firmware_version: state_update_packet.firmware_version,
            serial_number: state_update_packet.serial_number,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            button_configuration: state_update_packet.button_configuration,
            ambient_sound_mode_cycle: state_update_packet.ambient_sound_mode_cycle,
            sound_modes: state_update_packet.sound_modes,
            auto_power_off: state_update_packet.auto_power_off,
            limit_high_volume: state_update_packet.limit_high_volume,
            hear_id: state_update_packet.hear_id,
            voice_prompt: state_update_packet.voice_prompt,
            low_battery_prompt: state_update_packet.low_battery_prompt,
            ldac: state_update_packet.ldac,
            dual_connections: DualConnections {
                is_enabled: state_update_packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<A3040StateUpdatePacket> for A3040State {
    fn update(&mut self, partial: A3040StateUpdatePacket) {
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
            dual_connections_enabled,
        } = partial;
        self.battery_level = battery_level;
        self.firmware_version = firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.button_configuration = button_configuration;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.sound_modes = sound_modes;
        self.auto_power_off = auto_power_off;
        self.limit_high_volume = limit_high_volume;
        self.voice_prompt = voice_prompt;
        self.low_battery_prompt = low_battery_prompt;
        self.hear_id = hear_id;
        self.ldac = ldac;
        self.dual_connections.is_enabled = dual_connections_enabled;
    }
}

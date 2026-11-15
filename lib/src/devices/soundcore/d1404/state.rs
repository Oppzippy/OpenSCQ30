use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, DualConnections, DualConnectionsDevice,
            FirmwareVersion, Ldac, LowBatteryPrompt, SerialNumber, SingleBattery, SurroundSound,
        },
    },
    d1404,
};

use super::packets::inbound::D1404StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct D1404State {
    battery: SingleBattery,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: d1404::structures::D1404EqualizerConfiguration,
    button_configuration: d1404::structures::ButtonConfiguration,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    mfb_button_configuration: d1404::structures::MfbButtonConfiguration,
    sound_modes: d1404::structures::SoundModes,
    surround_sound: SurroundSound,
    ambient_sound_mode_voice_prompt: d1404::structures::AmbientSoundModeVoicePrompt,
    low_battery_prompt: LowBatteryPrompt,
    ldac: Ldac,
    dual_connections: DualConnections,
    auto_power_off: AutoPowerOff,
    button_configuration_reset: ResetButtonConfigurationPending,
}

impl D1404State {
    pub fn new(
        state_update_packet: D1404StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            battery: state_update_packet.battery,
            firmware_version: state_update_packet.firmware_version,
            serial_number: state_update_packet.serial_number,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            button_configuration: state_update_packet.button_configuration,
            ambient_sound_mode_cycle: state_update_packet.ambient_sound_mode_cycle,
            mfb_button_configuration: state_update_packet.mfb_button_configuration,
            sound_modes: state_update_packet.sound_modes,
            surround_sound: state_update_packet.surround_sound,
            ambient_sound_mode_voice_prompt: state_update_packet.ambient_sound_mode_voice_prompt,
            low_battery_prompt: state_update_packet.low_battery_prompt,
            ldac: state_update_packet.ldac,
            dual_connections: DualConnections {
                is_enabled: state_update_packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            auto_power_off: state_update_packet.auto_power_off,
            button_configuration_reset: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<D1404StateUpdatePacket> for D1404State {
    fn update(&mut self, partial: D1404StateUpdatePacket) {
        let D1404StateUpdatePacket {
            battery,
            firmware_version,
            serial_number,
            equalizer_configuration,
            button_configuration,
            ambient_sound_mode_cycle,
            mfb_button_configuration,
            sound_modes,
            surround_sound,
            ambient_sound_mode_voice_prompt,
            low_battery_prompt,
            ldac,
            dual_connections_enabled,
            auto_power_off,
        } = partial;
        self.battery = battery;
        self.firmware_version = firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.button_configuration = button_configuration;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.mfb_button_configuration = mfb_button_configuration;
        self.sound_modes = sound_modes;
        self.surround_sound = surround_sound;
        self.ambient_sound_mode_voice_prompt = ambient_sound_mode_voice_prompt;
        self.low_battery_prompt = low_battery_prompt;
        self.ldac = ldac;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.auto_power_off = auto_power_off;
    }
}

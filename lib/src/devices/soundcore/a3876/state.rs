use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3876,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AutoPowerOff, CommonEqualizerConfiguration, DualBatteryLevel, DualConnections,
            DualConnectionsDevice, DualFirmwareVersion, GamingMode, SerialNumber, TwsStatus,
            VoicePrompt, button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3876State {
    tws_status: TwsStatus,
    dual_battery_level: DualBatteryLevel,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    button_configuration: ButtonStatusCollection<8>,
    colorful_lights: a3876::structures::ColorfulLights,
    voice_prompt: VoicePrompt,
    auto_power_off: AutoPowerOff,
    gaming_mode: GamingMode,
    volume_balance: a3876::structures::VolumeBalance,
    dual_connections: DualConnections,
    reset_button_configuration_pending: ResetButtonConfigurationPending,
}

impl A3876State {
    pub fn new(
        packet: a3876::packets::inbound::A3876StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            tws_status: packet.tws_status,
            dual_battery_level: packet.dual_battery_level,
            dual_firmware_version: packet.dual_firmware_version,
            serial_number: packet.serial_number,
            equalizer_configuration: packet.equalizer_configuration,
            button_configuration: packet.button_configuration,
            colorful_lights: packet.colorful_lights,
            voice_prompt: packet.voice_prompt,
            auto_power_off: packet.auto_power_off,
            gaming_mode: packet.gaming_mode,
            volume_balance: packet.volume_balance,
            dual_connections: DualConnections {
                is_enabled: packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            reset_button_configuration_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<a3876::packets::inbound::A3876StateUpdatePacket> for A3876State {
    fn update(&mut self, partial: a3876::packets::inbound::A3876StateUpdatePacket) {
        let a3876::packets::inbound::A3876StateUpdatePacket {
            tws_status,
            dual_battery_level,
            dual_firmware_version,
            serial_number,
            equalizer_configuration,
            button_configuration,
            colorful_lights,
            voice_prompt,
            auto_power_off,
            gaming_mode,
            volume_balance,
            dual_connections_enabled,
        } = partial;

        self.tws_status = tws_status;
        self.dual_battery_level = dual_battery_level;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.button_configuration = button_configuration;
        self.colorful_lights = colorful_lights;
        self.voice_prompt = voice_prompt;
        self.auto_power_off = auto_power_off;
        self.gaming_mode = gaming_mode;
        self.volume_balance = volume_balance;
        self.dual_connections.is_enabled = dual_connections_enabled;
    }
}

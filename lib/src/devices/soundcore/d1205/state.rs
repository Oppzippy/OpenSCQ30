use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AutoPowerOff, CaseBatteryLevel, DualBattery, DualConnections, DualConnectionsDevice,
            DualFirmwareVersion, GamingMode, Ldac, LowBatteryPrompt, SerialNumber, TouchTone,
            TwsStatus, WearingDetection, WearingTone, button_configuration::ButtonStatusCollection,
        },
    },
    d1205::{
        modules::equalizer::equalizer_configuration_from_block,
        packets::inbound::D1205StateUpdatePacket,
        structures::{D1205EqualizerConfiguration, SoundModes, SpatialAudio},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct D1205State {
    tws_status: TwsStatus,
    battery: DualBattery,
    firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    case_battery_level: CaseBatteryLevel,
    equalizer_configuration: D1205EqualizerConfiguration,
    button_configuration: ButtonStatusCollection<8>,
    sound_modes: SoundModes,
    touch_tone: TouchTone,
    low_battery_prompt: LowBatteryPrompt,
    ldac: Ldac,
    dual_connections: DualConnections,
    auto_power_off: AutoPowerOff,
    spatial_audio: SpatialAudio,
    wearing_detection: WearingDetection,
    wearing_tone: WearingTone,
    gaming_mode: GamingMode,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl D1205State {
    pub fn new(
        packet: D1205StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            tws_status: packet.tws_status,
            battery: packet.battery,
            firmware_version: packet.firmware_version,
            serial_number: packet.serial_number,
            case_battery_level: packet.case_battery_level,
            equalizer_configuration: equalizer_configuration_from_block(&packet.equalizer),
            button_configuration: packet.button_configuration,
            sound_modes: packet.sound_modes,
            touch_tone: packet.touch_tone,
            low_battery_prompt: packet.low_battery_prompt,
            ldac: packet.ldac,
            dual_connections: DualConnections {
                is_enabled: packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            auto_power_off: packet.auto_power_off,
            spatial_audio: packet.spatial_audio,
            wearing_detection: packet.wearing_detection,
            wearing_tone: packet.wearing_tone,
            gaming_mode: packet.gaming_mode,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<D1205StateUpdatePacket> for D1205State {
    fn update(&mut self, partial: D1205StateUpdatePacket) {
        self.tws_status = partial.tws_status;
        self.battery = partial.battery;
        self.firmware_version = partial.firmware_version;
        self.serial_number = partial.serial_number;
        self.case_battery_level = partial.case_battery_level;
        self.equalizer_configuration = equalizer_configuration_from_block(&partial.equalizer);
        self.button_configuration = partial.button_configuration;
        self.sound_modes = partial.sound_modes;
        self.touch_tone = partial.touch_tone;
        self.low_battery_prompt = partial.low_battery_prompt;
        self.ldac = partial.ldac;
        self.dual_connections.is_enabled = partial.dual_connections_enabled;
        self.auto_power_off = partial.auto_power_off;
        self.spatial_audio = partial.spatial_audio;
        self.wearing_detection = partial.wearing_detection;
        self.wearing_tone = partial.wearing_tone;
        self.gaming_mode = partial.gaming_mode;
    }
}

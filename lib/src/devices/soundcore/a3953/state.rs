use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3953,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel, CommonEqualizerConfiguration,
            CustomHearId, DualBattery, DualConnections, DualConnectionsDevice, DualFirmwareVersion,
            Ldac, LowBatteryPrompt, SerialNumber, TwsStatus, WearingDetection, WearingTone,
            button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3953State {
    tws_status: TwsStatus,
    battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    is_hear_id_initialized: a3953::structures::IsHearIdInitialized,
    hear_id: CustomHearId<2, 10>,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3953::structures::SoundModes,
    wearing_detection: WearingDetection,
    case_battery_level: CaseBatteryLevel,
    ldac: Ldac,
    dual_connections: DualConnections,
    auto_power_off: AutoPowerOff,
    wearing_tone: WearingTone,
    low_battery_prompt: LowBatteryPrompt,
    ambient_sound_prompt: a3953::structures::AmbientSoundPrompt,
    spatial_audio: a3953::structures::SpatialAudio,
    press_sensitivity: a3953::structures::PressSensitivity,
    button_configuration: ButtonStatusCollection<8>,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl A3953State {
    pub fn new(
        packet: a3953::packets::A3953StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            tws_status: packet.tws_status,
            battery: packet.battery,
            dual_firmware_version: packet.dual_firmware_version,
            serial_number: packet.serial_number,
            equalizer_configuration: packet.equalizer_configuration,
            is_hear_id_initialized: packet.is_hear_id_initialized,
            hear_id: packet.hear_id,
            ambient_sound_mode_cycle: packet.ambient_sound_mode_cycle,
            sound_modes: packet.sound_modes,
            wearing_detection: packet.wearing_detection,
            case_battery_level: packet.case_battery_level,
            ldac: packet.ldac,
            dual_connections: DualConnections {
                is_enabled: packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            auto_power_off: packet.auto_power_off,
            wearing_tone: packet.wearing_tone,
            low_battery_prompt: packet.low_battery_prompt,
            ambient_sound_prompt: packet.ambient_sound_prompt,
            spatial_audio: packet.spatial_audio,
            press_sensitivity: packet.press_sensitivity.unwrap_or_default(),
            button_configuration: packet.button_configuration,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

// Custom instead of the usual blanket From-based Update: a full state update packet doesn't carry
// the dual connections device list (that's fetched separately), so a plain replace would wipe it
// out on every subsequent state refresh. Only dual_connections.is_enabled gets refreshed here.
impl Update<a3953::packets::A3953StateUpdatePacket> for A3953State {
    fn update(&mut self, packet: a3953::packets::A3953StateUpdatePacket) {
        let a3953::packets::A3953StateUpdatePacket {
            tws_status,
            battery,
            dual_firmware_version,
            serial_number,
            equalizer_configuration,
            is_hear_id_initialized,
            hear_id,
            custom_length: _,
            button_configuration,
            unknown_gap: _,
            ambient_sound_mode_cycle,
            sound_modes,
            wearing_detection,
            case_battery_level,
            ldac,
            dual_connections_enabled,
            auto_power_off,
            wearing_tone,
            low_battery_prompt,
            ambient_sound_prompt,
            spatial_audio,
            device_colour: _,
            press_sensitivity,
            unknown_suffix: _,
        } = packet;

        self.tws_status = tws_status;
        self.battery = battery;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.is_hear_id_initialized = is_hear_id_initialized;
        self.hear_id = hear_id;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.sound_modes = sound_modes;
        self.wearing_detection = wearing_detection;
        self.case_battery_level = case_battery_level;
        self.ldac = ldac;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.auto_power_off = auto_power_off;
        self.wearing_tone = wearing_tone;
        self.low_battery_prompt = low_battery_prompt;
        self.ambient_sound_prompt = ambient_sound_prompt;
        self.spatial_audio = spatial_audio;
        self.press_sensitivity = press_sensitivity.unwrap_or_default();
        self.button_configuration = button_configuration;
    }
}

use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3954,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AgeRange, AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel,
            CommonEqualizerConfiguration, CustomHearId, DualBattery, DualConnections,
            DualConnectionsDevice, DualFirmwareVersion, Gender, Ldac, LimitHighVolume,
            LowBatteryPrompt, SerialNumber, SoundLeakCompensation, TwsStatus, WearingDetection,
            button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3954State {
    tws_status: TwsStatus,
    battery: DualBattery,
    firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    case_firmware_version: a3954::structures::CaseFirmwareVersion,
    case_battery_level: CaseBatteryLevel,
    case_serial_number: a3954::structures::CaseSerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    hear_id: CustomHearId<2, 10>,
    button_configuration: ButtonStatusCollection<12>,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3954::structures::SoundModes,
    case_features: a3954::structures::CaseFeatures,
    air_pressure: a3954::structures::AirPressure,
    low_battery_prompt: LowBatteryPrompt,
    ldac: Ldac,
    dual_connections: DualConnections,
    auto_power_off: AutoPowerOff,
    limit_high_volume: LimitHighVolume,
    spatial_audio: a3954::structures::SpatialAudio,
    easy_chat: a3954::structures::EasyChat,
    sound_leak_compensation: SoundLeakCompensation,
    case_language: a3954::structures::CaseLanguage,
    wearing_detection: WearingDetection,
    gender: Gender,
    age_range: AgeRange,
    reset_button_configuration_pending: ResetButtonConfigurationPending,
}

impl A3954State {
    pub fn new(
        packet: a3954::packets::inbound::A3954StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            tws_status: packet.tws_status,
            battery: packet.battery,
            firmware_version: packet.firmware_version,
            serial_number: packet.serial_number,
            case_firmware_version: packet.case_firmware_version,
            case_battery_level: packet.case_battery_level,
            case_serial_number: packet.case_serial_number,
            equalizer_configuration: packet.equalizer_configuration,
            hear_id: packet.hear_id,
            button_configuration: packet.button_configuration,
            ambient_sound_mode_cycle: packet.ambient_sound_mode_cycle,
            sound_modes: packet.sound_modes,
            case_features: packet.case_features,
            air_pressure: packet.air_pressure,
            low_battery_prompt: packet.low_battery_prompt,
            ldac: packet.ldac,
            dual_connections: DualConnections {
                is_enabled: packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            auto_power_off: packet.auto_power_off,
            limit_high_volume: packet.limit_high_volume,
            spatial_audio: packet.spatial_audio,
            easy_chat: packet.easy_chat,
            sound_leak_compensation: packet.sound_leak_compensation,
            case_language: packet.case_language,
            wearing_detection: packet.wearing_detection,
            gender: Gender::default(),
            age_range: AgeRange::default(),
            reset_button_configuration_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<a3954::packets::inbound::A3954StateUpdatePacket> for A3954State {
    fn update(&mut self, partial: a3954::packets::inbound::A3954StateUpdatePacket) {
        let a3954::packets::inbound::A3954StateUpdatePacket {
            tws_status,
            battery,
            firmware_version,
            serial_number,
            case_firmware_version,
            case_battery_level,
            case_serial_number,
            equalizer_configuration,
            hear_id,
            button_configuration,
            ambient_sound_mode_cycle,
            sound_modes,
            case_features,
            air_pressure,
            low_battery_prompt,
            ldac,
            dual_connections_enabled,
            auto_power_off,
            limit_high_volume,
            spatial_audio,
            easy_chat,
            sound_leak_compensation,
            case_language,
            wearing_detection,
        } = partial;

        self.tws_status = tws_status;
        self.battery = battery;
        self.firmware_version = firmware_version;
        self.serial_number = serial_number;
        self.case_firmware_version = case_firmware_version;
        self.case_battery_level = case_battery_level;
        self.case_serial_number = case_serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.sound_modes = sound_modes;
        self.case_features = case_features;
        self.air_pressure = air_pressure;
        self.low_battery_prompt = low_battery_prompt;
        self.ldac = ldac;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.auto_power_off = auto_power_off;
        self.limit_high_volume = limit_high_volume;
        self.spatial_audio = spatial_audio;
        self.easy_chat = easy_chat;
        self.sound_leak_compensation = sound_leak_compensation;
        self.case_language = case_language;
        self.wearing_detection = wearing_detection;
    }
}

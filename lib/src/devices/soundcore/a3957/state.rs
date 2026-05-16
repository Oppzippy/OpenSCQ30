use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3957::{self, packets::inbound::A3957StateUpdatePacket},
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AgeRange, AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel,
            CommonEqualizerConfiguration, CustomHearId, DualBattery, DualConnections,
            DualConnectionsDevice, DualFirmwareVersion, GamingMode, Gender, Ldac, LimitHighVolume,
            LowBatteryPrompt, SerialNumber, SoundLeakCompensation, TouchTone, TwsStatus,
            WearingDetection, WearingTone, button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3957State {
    tws_status: TwsStatus,
    dual_battery: DualBattery,
    case_battery: CaseBatteryLevel,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    age_range: AgeRange,
    gender: Gender,
    hear_id: CustomHearId<2, 10>,
    button_configuration: ButtonStatusCollection<8>,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3957::structures::SoundModes,
    dual_connections: DualConnections,
    ldac: Ldac,
    wearing_tone: WearingTone,
    auto_power_off: AutoPowerOff,
    limit_high_volume: LimitHighVolume,
    touch_tone: TouchTone,
    low_battery_prompt: LowBatteryPrompt,
    immersive_experience: a3957::structures::ImmersiveExperience,
    sound_leak_compensation: SoundLeakCompensation,
    wearing_detection: WearingDetection,
    gaming_mode: GamingMode,
    pressure_sensitivity: a3957::structures::PressureSensitivity,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl A3957State {
    pub fn new(
        packet: a3957::packets::inbound::A3957StateUpdatePacket,
        dual_connections_devices: Vec<Option<DualConnectionsDevice>>,
    ) -> Self {
        Self {
            tws_status: packet.tws_status,
            dual_battery: packet.dual_battery,
            case_battery: packet.case_battery,
            dual_firmware_version: packet.dual_firmware_version,
            serial_number: packet.serial_number,
            equalizer_configuration: packet.equalizer_configuration,
            age_range: packet.age_range,
            gender: packet.gender,
            hear_id: packet.hear_id,
            button_configuration: packet.button_configuration,
            ambient_sound_mode_cycle: packet.ambient_sound_mode_cycle,
            sound_modes: packet.sound_modes,
            dual_connections: DualConnections {
                is_enabled: packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            ldac: packet.ldac,
            wearing_tone: packet.wearing_tone,
            auto_power_off: packet.auto_power_off,
            limit_high_volume: packet.limit_high_volume,
            touch_tone: packet.touch_tone,
            low_battery_prompt: packet.low_battery_prompt,
            immersive_experience: packet.immersive_experience,
            sound_leak_compensation: packet.sound_leak_compensation,
            wearing_detection: packet.wearing_detection,
            gaming_mode: packet.gaming_mode,
            pressure_sensitivity: packet.pressure_sensitivity,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<A3957StateUpdatePacket> for A3957State {
    fn update(&mut self, partial: A3957StateUpdatePacket) {
        let A3957StateUpdatePacket {
            tws_status,
            dual_battery,
            dual_firmware_version,
            serial_number,
            case_battery,
            equalizer_configuration,
            age_range,
            gender,
            hear_id,
            button_configuration,
            ambient_sound_mode_cycle,
            sound_modes,
            wearing_tone,
            low_battery_prompt,
            ldac,
            dual_connections_enabled,
            auto_power_off,
            limit_high_volume,
            immersive_experience,
            sound_leak_compensation,
            wearing_detection,
            touch_tone,
            gaming_mode,
            pressure_sensitivity,
        } = partial;
        self.tws_status = tws_status;
        self.dual_battery = dual_battery;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.case_battery = case_battery;
        self.equalizer_configuration = equalizer_configuration;
        self.age_range = age_range;
        self.gender = gender;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.sound_modes = sound_modes;
        self.wearing_tone = wearing_tone;
        self.low_battery_prompt = low_battery_prompt;
        self.ldac = ldac;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.auto_power_off = auto_power_off;
        self.limit_high_volume = limit_high_volume;
        self.immersive_experience = immersive_experience;
        self.sound_leak_compensation = sound_leak_compensation;
        self.wearing_detection = wearing_detection;
        self.touch_tone = touch_tone;
        self.gaming_mode = gaming_mode;
        self.pressure_sensitivity = pressure_sensitivity;
    }
}

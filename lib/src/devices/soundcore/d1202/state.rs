use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AgeRange, AutoPowerOff, CaseBatteryLevel, CommonEqualizerConfiguration, CustomHearId,
            DualBattery, DualConnections, DualConnectionsDevice, DualFirmwareVersion, Gender, Ldac,
            LimitHighVolume, LowBatteryPrompt, SerialNumber, TouchTone, TwsStatus,
            button_configuration::ButtonStatusCollection,
        },
    },
    d1202,
};

use super::packets::inbound::D1202StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct D1202State {
    tws_status: TwsStatus,
    battery: DualBattery,
    firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    case_battery_level: CaseBatteryLevel,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    hear_id: CustomHearId<2, 10>,
    button_configuration: ButtonStatusCollection<8>,
    sound_modes: d1202::structures::SoundModes,
    touch_tone: TouchTone,
    low_battery_prompt: LowBatteryPrompt,
    ldac: Ldac,
    dual_connections: DualConnections,
    auto_power_off: AutoPowerOff,
    limit_high_volume: LimitHighVolume,
    spatial_audio: d1202::structures::SpatialAudio,
    gender: Gender,
    age_range: AgeRange,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl D1202State {
    pub fn new(
        packet: D1202StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            tws_status: packet.tws_status,
            battery: packet.battery,
            firmware_version: packet.firmware_version,
            serial_number: packet.serial_number,
            case_battery_level: packet.case_battery_level,
            equalizer_configuration: packet.equalizer_configuration,
            hear_id: packet.hear_id,
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
            limit_high_volume: packet.limit_high_volume,
            spatial_audio: packet.spatial_audio,
            gender: Gender::default(),
            age_range: AgeRange::default(),
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<D1202StateUpdatePacket> for D1202State {
    fn update(&mut self, partial: D1202StateUpdatePacket) {
        let D1202StateUpdatePacket {
            tws_status,
            battery,
            firmware_version,
            serial_number,
            case_battery_level,
            equalizer_configuration,
            hear_id,
            button_configuration,
            sound_modes,
            touch_tone,
            low_battery_prompt,
            ldac,
            dual_connections_enabled,
            auto_power_off,
            limit_high_volume,
            spatial_audio,
        } = partial;

        self.tws_status = tws_status;
        self.battery = battery;
        self.firmware_version = firmware_version;
        self.serial_number = serial_number;
        self.case_battery_level = case_battery_level;
        self.equalizer_configuration = equalizer_configuration;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
        self.sound_modes = sound_modes;
        self.touch_tone = touch_tone;
        self.low_battery_prompt = low_battery_prompt;
        self.ldac = ldac;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.auto_power_off = auto_power_off;
        self.limit_high_volume = limit_high_volume;
        self.spatial_audio = spatial_audio;
    }
}

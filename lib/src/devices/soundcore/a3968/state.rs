use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3968,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AgeRange, AmbientSoundModeCycleTws, AutoPowerOff, CaseBatteryLevel,
            CommonEqualizerConfiguration, CustomHearId, DualBattery, DualConnections,
            DualConnectionsDevice, DualFirmwareVersion, Gender, SerialNumber, TouchTone, TwsStatus,
            button_configuration::ButtonStatusCollection,
        },
    },
};

use super::packets::inbound::A3968StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3968State {
    tws_status: TwsStatus,
    dual_battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    case_battery_level: CaseBatteryLevel,
    sound_modes: a3968::structures::SoundModes,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    hear_id: CustomHearId<2, 10>,
    button_configuration: ButtonStatusCollection<6>,
    ambient_sound_mode_cycle: AmbientSoundModeCycleTws,
    dual_connections: DualConnections,
    touch_tone: TouchTone,
    auto_power_off: AutoPowerOff,
    age_range: AgeRange,
    gender: Gender,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl A3968State {
    pub fn new(
        packet: a3968::packets::inbound::A3968StateUpdatePacket,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            tws_status: packet.tws_status,
            dual_battery: packet.dual_battery,
            dual_firmware_version: packet.dual_firmware_version,
            serial_number: packet.serial_number,
            case_battery_level: packet.case_battery_level,
            sound_modes: packet.sound_modes,
            equalizer_configuration: packet.equalizer_configuration,
            hear_id: packet.hear_id,
            button_configuration: packet.button_configuration,
            ambient_sound_mode_cycle: packet.ambient_sound_mode_cycle,
            dual_connections: DualConnections {
                is_enabled: packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
            touch_tone: packet.touch_tone,
            auto_power_off: packet.auto_power_off,
            age_range: AgeRange::default(),
            gender: Gender::default(),
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<A3968StateUpdatePacket> for A3968State {
    fn update(&mut self, partial: A3968StateUpdatePacket) {
        let A3968StateUpdatePacket {
            tws_status,
            dual_battery,
            dual_firmware_version,
            serial_number,
            case_battery_level,
            equalizer_configuration,
            hear_id,
            button_configuration,
            sound_modes,
            dual_connections_enabled,
            touch_tone,
            auto_power_off,
            ambient_sound_mode_cycle,
        } = partial;
        self.tws_status = tws_status;
        self.dual_battery = dual_battery;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.case_battery_level = case_battery_level;
        self.equalizer_configuration = equalizer_configuration;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
        self.sound_modes = sound_modes;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.touch_tone = touch_tone;
        self.auto_power_off = auto_power_off;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
    }
}

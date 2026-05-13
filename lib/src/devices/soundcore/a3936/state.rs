use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    modules::reset_button_configuration::ResetButtonConfigurationPending,
    state::Update,
    structures::{
        AgeRange, AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel,
        CommonEqualizerConfiguration, CustomHearId, DualBattery, DualConnections,
        DualFirmwareVersion, GamingMode, Gender, Ldac, SerialNumber, TouchTone, TwsStatus,
        button_configuration::ButtonStatusCollection,
    },
};

use super::{packets::A3936StateUpdatePacket, structures::A3936SoundModes};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3936State {
    tws_status: TwsStatus,
    battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    age_range: AgeRange,
    custom_hear_id: CustomHearId<2, 10>,
    sound_modes: A3936SoundModes,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    button_configuration: ButtonStatusCollection<6>,
    case_battery_level: CaseBatteryLevel,
    auto_power_off: AutoPowerOff,
    gender: Gender,
    touch_tone: TouchTone,
    gaming_mode: GamingMode,
    ldac: Ldac,
    #[has(skip)]
    color: u8,
    dual_connections: DualConnections,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl A3936State {
    pub fn new(state_update_packet: A3936StateUpdatePacket) -> Self {
        Self {
            tws_status: state_update_packet.tws_status,
            battery: state_update_packet.battery,
            dual_firmware_version: state_update_packet.dual_firmware_version,
            serial_number: state_update_packet.serial_number,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            age_range: state_update_packet.age_range,
            custom_hear_id: state_update_packet.custom_hear_id,
            sound_modes: state_update_packet.sound_modes,
            ambient_sound_mode_cycle: state_update_packet.ambient_sound_mode_cycle,
            button_configuration: state_update_packet.button_configuration,
            touch_tone: state_update_packet.touch_tone,
            case_battery_level: state_update_packet.case_battery_level,
            color: state_update_packet.color,
            ldac: state_update_packet.ldac,
            dual_connections: DualConnections {
                is_enabled: state_update_packet.dual_connections_enabled,
                devices: Vec::new(),
            },
            auto_power_off: state_update_packet.auto_power_off,
            gaming_mode: state_update_packet.gaming_mode,
            gender: Gender::default(),
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}

impl Update<A3936StateUpdatePacket> for A3936State {
    fn update(&mut self, partial: A3936StateUpdatePacket) {
        let A3936StateUpdatePacket {
            tws_status,
            battery,
            dual_firmware_version,
            serial_number,
            equalizer_configuration,
            age_range,
            custom_hear_id,
            sound_modes,
            ambient_sound_mode_cycle,
            button_configuration,
            touch_tone,
            case_battery_level,
            color,
            ldac,
            dual_connections_enabled,
            auto_power_off,
            gaming_mode,
        } = partial;
        self.tws_status = tws_status;
        self.battery = battery;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.age_range = age_range;
        self.custom_hear_id = custom_hear_id;
        self.sound_modes = sound_modes;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.button_configuration = button_configuration;
        self.touch_tone = touch_tone;
        self.case_battery_level = case_battery_level;
        self.color = color;
        self.ldac = ldac;
        self.dual_connections.is_enabled = dual_connections_enabled;
        self.auto_power_off = auto_power_off;
        self.gaming_mode = gaming_mode;
    }
}

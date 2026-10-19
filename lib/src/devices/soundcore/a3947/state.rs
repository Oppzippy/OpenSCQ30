use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3947,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        state::Update,
        structures::{
            AmbientSoundModeCycle, AutoPlayPause, AutoPowerOff, CaseBatteryLevel,
            CommonEqualizerConfiguration, DualBattery, DualConnections, DualConnectionsDevice,
            DualFirmwareVersion, GamingMode, Ldac, LimitHighVolume, LowBatteryPrompt, SerialNumber,
            SoundLeakCompensation, SurroundSound, TouchLock, TouchTone, TwsStatus, WearingTone,
            button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3947State {
    tws_status: TwsStatus,
    battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    hear_id: a3947::structures::HearId<2, 10>,
    button_configuration: ButtonStatusCollection<8>,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3947::structures::SoundModes,
    touch_tone: TouchTone,
    limit_high_volume: LimitHighVolume,
    auto_power_off: AutoPowerOff,
    case_battery_level: CaseBatteryLevel,
    gaming_mode: GamingMode,
    sound_leak_compensation: SoundLeakCompensation,
    surround_sound: SurroundSound,
    auto_play_pause: AutoPlayPause,
    wearing_tone: WearingTone,
    touch_lock: TouchLock,
    low_battery_prompt: LowBatteryPrompt,
    reset_button_configuration_pending: ResetButtonConfigurationPending,
    ldac: Ldac,
    dual_connections: DualConnections,
}

impl A3947State {
    pub fn new(
        state_update_packet: a3947::packets::A3947StateUpdatePacket,
        ldac: Ldac,
        dual_connections_devices: Vec<DualConnectionsDevice>,
    ) -> Self {
        Self {
            tws_status: state_update_packet.tws_status,
            battery: state_update_packet.battery,
            dual_firmware_version: state_update_packet.dual_firmware_version,
            serial_number: state_update_packet.serial_number,
            equalizer_configuration: state_update_packet.equalizer_configuration,
            hear_id: state_update_packet.hear_id,
            button_configuration: state_update_packet.button_configuration,
            ambient_sound_mode_cycle: state_update_packet.ambient_sound_mode_cycle,
            sound_modes: state_update_packet.sound_modes,
            touch_tone: state_update_packet.touch_tone,
            limit_high_volume: state_update_packet.limit_high_volume,
            auto_power_off: state_update_packet.auto_power_off,
            case_battery_level: state_update_packet.case_battery_level,
            gaming_mode: state_update_packet.gaming_mode,
            sound_leak_compensation: state_update_packet.sound_leak_compensation,
            surround_sound: state_update_packet.surround_sound,
            auto_play_pause: state_update_packet.auto_play_pause,
            wearing_tone: state_update_packet.wearing_tone,
            touch_lock: state_update_packet.touch_lock,
            low_battery_prompt: state_update_packet.low_battery_prompt,
            reset_button_configuration_pending: ResetButtonConfigurationPending::default(),
            ldac,
            dual_connections: DualConnections {
                is_enabled: state_update_packet.dual_connections_enabled,
                devices: dual_connections_devices,
            },
        }
    }
}

impl Update<a3947::packets::A3947StateUpdatePacket> for A3947State {
    fn update(&mut self, partial: a3947::packets::A3947StateUpdatePacket) {
        let a3947::packets::A3947StateUpdatePacket {
            tws_status,
            battery,
            dual_firmware_version,
            serial_number,
            equalizer_configuration,
            hear_id,
            button_configuration,
            ambient_sound_mode_cycle,
            sound_modes,
            case_battery_level,
            sound_leak_compensation,
            gaming_mode,
            touch_tone,
            surround_sound,
            limit_high_volume,
            auto_play_pause,
            wearing_tone,
            auto_power_off,
            touch_lock,
            low_battery_prompt,
            dual_connections_enabled,
        } = partial;

        self.tws_status = tws_status;
        self.battery = battery;
        self.dual_firmware_version = dual_firmware_version;
        self.serial_number = serial_number;
        self.equalizer_configuration = equalizer_configuration;
        self.hear_id = hear_id;
        self.button_configuration = button_configuration;
        self.ambient_sound_mode_cycle = ambient_sound_mode_cycle;
        self.sound_modes = sound_modes;
        self.case_battery_level = case_battery_level;
        self.sound_leak_compensation = sound_leak_compensation;
        self.gaming_mode = gaming_mode;
        self.touch_tone = touch_tone;
        self.surround_sound = surround_sound;
        self.limit_high_volume = limit_high_volume;
        self.auto_play_pause = auto_play_pause;
        self.wearing_tone = wearing_tone;
        self.auto_power_off = auto_power_off;
        self.touch_lock = touch_lock;
        self.low_battery_prompt = low_battery_prompt;
        self.dual_connections.is_enabled = dual_connections_enabled;
    }
}

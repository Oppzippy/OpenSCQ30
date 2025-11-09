use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3947,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        structures::{
            AmbientSoundModeCycle, AutoPlayPause, AutoPowerOff, CaseBatteryLevel, DualBattery,
            DualFirmwareVersion, EqualizerConfiguration, GamingMode, LimitHighVolume,
            LowBatteryPrompt, SerialNumber, SoundLeakCompensation, SurroundSound, TouchLock,
            TouchTone, TwsStatus, WearingTone, button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3947State {
    tws_status: TwsStatus,
    battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: EqualizerConfiguration<2, 10>,
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
}

impl From<a3947::packets::A3947StateUpdatePacket> for A3947State {
    fn from(packet: a3947::packets::A3947StateUpdatePacket) -> Self {
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
        } = packet;

        Self {
            reset_button_configuration_pending: ResetButtonConfigurationPending::default(),
            tws_status,
            battery,
            dual_firmware_version,
            serial_number,
            equalizer_configuration,
            hear_id,
            button_configuration,
            ambient_sound_mode_cycle,
            sound_modes,
            touch_tone,
            limit_high_volume,
            auto_power_off,
            case_battery_level,
            gaming_mode,
            sound_leak_compensation,
            surround_sound,
            auto_play_pause,
            wearing_tone,
            touch_lock,
            low_battery_prompt,
        }
    }
}

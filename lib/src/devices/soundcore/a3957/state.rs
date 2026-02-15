use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3957,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        structures::{
            AgeRange, AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel,
            CommonEqualizerConfiguration, CustomHearId, DualBattery, DualFirmwareVersion,
            GamingMode, Gender, LimitHighVolume, LowBatteryPrompt, SerialNumber,
            SoundLeakCompensation, TouchTone, TwsStatus, WearingDetection, WearingTone,
            button_configuration::ButtonStatusCollection,
        },
    },
};

use super::structures::{
    AncPersonalizedToEarCanal, ImmersiveExperience, PressureSensitivity, SoundModes,
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
    sound_modes: SoundModes,
    anc_personalized_to_ear_canal: AncPersonalizedToEarCanal,
    ldac: bool,
    wearing_tone: WearingTone,
    auto_power_off: AutoPowerOff,
    limit_high_volume: LimitHighVolume,
    touch_tone: TouchTone,
    low_battery_prompt: LowBatteryPrompt,
    immersive_experience: ImmersiveExperience,
    sound_leak_compensation: SoundLeakCompensation,
    wearing_detection: WearingDetection,
    gaming_mode: GamingMode,
    pressure_sensitivity: PressureSensitivity,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl From<a3957::packets::inbound::A3957StateUpdatePacket> for A3957State {
    fn from(packet: a3957::packets::inbound::A3957StateUpdatePacket) -> Self {
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
            anc_personalized_to_ear_canal: packet.anc_personalized_to_ear_canal,
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

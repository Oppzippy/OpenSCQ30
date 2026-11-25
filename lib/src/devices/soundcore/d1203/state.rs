use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    d1203::{
        modules::{
            D1203AmbientSoundMode, D1203AutoPowerOff, D1203ButtonConfiguration,
            D1203CustomEqualizer, D1203EasyChat, D1203Ldac, D1203LowBatteryPrompt,
            D1203ManualNoiseCanceling, D1203Multipoint, D1203NoiseCancelingMode, D1203Soundstage,
            D1203TouchLock, D1203TouchSensitivity, D1203TouchTone, D1203TransportationMode,
            D1203WearDetection, D1203WearingTone, D1203WindNoise,
        },
        packets::inbound::{D1203PresetEqualizer, D1203StateUpdatePacket},
    },
    common::structures::{CaseBatteryLevel, DualBattery, DualFirmwareVersion, SerialNumber},
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct D1203State {
    dual_battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    #[has(skip)]
    preset_equalizer: D1203PresetEqualizer,
    /// User-requested custom 8-band EQ gains (whole dB). Written by the
    /// interactive `customEqualizerProfile` setting; see `modules`.
    custom_equalizer: D1203CustomEqualizer,
    /// Ambient sound mode (ANC / Transparency / Normal / Adaptive). The D1203
    /// write command (`Command([0x06, 0x81])`, 7-byte payload) is identical to
    /// the A3957, but the D1203 has a FOURTH mode (`3 = Adaptive`) the A3957
    /// enum cannot represent, so we use a D1203-specific enum instead of
    /// reusing `a3957::structures::SoundModes`. See `modules::ambient_sound_mode`.
    ambient_sound_mode: D1203AmbientSoundMode,
    /// Noise-canceling sub-mode (byte 1 of the `06 81` payload): Manual or
    /// Transportation. Only meaningful when the ambient mode is NoiseCanceling,
    /// but the device always carries a value here.
    nc_mode: D1203NoiseCancelingMode,
    /// Manual ANC level (byte 3 of the `06 81` payload), 1..=5.
    manual_nc_level: D1203ManualNoiseCanceling,
    /// Transport type (Airplane / Car) that drives byte[5] of the `06 81`
    /// payload WHILE the NC sub-mode is Transportation. Outside Transportation
    /// byte[5] is the active flag and this field is ignored; see
    /// `modules::ambient_sound_mode::build_sound_mode_payload`.
    transportation_mode: D1203TransportationMode,
    /// Wind noise reduction toggle. Driven by its OWN dedicated command
    /// (`Command([0x41, 0x88])`, payload `[0x0b, 0|1]`), NOT the `06 81`
    /// sound-mode command. See `modules::wind_noise`.
    wind_noise: D1203WindNoise,
    /// Current action of each of the eight configurable buttons (2 sides x 4
    /// gestures). Driven by the `04 8c` command, one packet per changed gesture.
    /// See `modules::button_configuration`.
    button_configuration: D1203ButtonConfiguration,
    /// Touch Tone prompt toggle. Driven by command `41 87` sub-id 0. See
    /// `modules::misc`.
    touch_tone: D1203TouchTone,
    /// Low Battery Prompt toggle. Driven by command `41 87` sub-id 1.
    low_battery_prompt: D1203LowBatteryPrompt,
    /// Wearing Tone prompt toggle. Driven by command `41 87` sub-id 3.
    wearing_tone: D1203WearingTone,
    /// LDAC high-resolution codec toggle. Driven by command `41 88` sub-id 2.
    ldac: D1203Ldac,
    /// Easy Chat toggle. Driven by command `41 88` sub-id 14.
    easy_chat: D1203EasyChat,
    /// Soundstage / Soundfield rendering toggle. Driven by command `41 88`
    /// sub-id 29. Exposed via `SettingId::SpatialAudio`.
    soundstage: D1203Soundstage,
    /// Auto power off timeout selector. Driven by command `01 86`, payload
    /// `[enabled, duration_idx]`. See `modules::misc`.
    auto_power_off: D1203AutoPowerOff,
    /// Case battery, state tag 8.
    case_battery_level: CaseBatteryLevel,
    /// Touch controls locked (`41 88` sub-id 0, inverted on the wire).
    touch_lock: D1203TouchLock,
    /// Multipoint (`41 88` sub-id 3).
    multipoint: D1203Multipoint,
    /// Wear detection (`41 88` sub-id 8).
    wear_detection: D1203WearDetection,
    /// Touch sensitivity, state tag 58 and `04 85`.
    touch_sensitivity: D1203TouchSensitivity,
}

/// Which read command a switch tag belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchFamily {
    /// `41 07`: tap tone (0), low battery prompt (1), wearing tone (3).
    Tones,
    /// `41 08`: touch controls (0, inverted: 1 means locked), multipoint (3),
    /// wear detection (8), sound field (0x1d).
    Switches,
}

impl SwitchFamily {
    pub fn command(self) -> crate::devices::soundcore::common::packet::Command {
        match self {
            Self::Tones => crate::devices::soundcore::common::packet::Command([0x41, 0x07]),
            Self::Switches => crate::devices::soundcore::common::packet::Command([0x41, 0x08]),
        }
    }
}

impl D1203State {
    /// Applies the answer to a `41 07` / `41 08` read.
    pub fn apply_switch(&mut self, family: SwitchFamily, tag: u8, value: bool) {
        match (family, tag) {
            (SwitchFamily::Tones, 0x00) => self.touch_tone = D1203TouchTone(value),
            (SwitchFamily::Tones, 0x01) => self.low_battery_prompt = D1203LowBatteryPrompt(value),
            (SwitchFamily::Tones, 0x03) => self.wearing_tone = D1203WearingTone(value),
            (SwitchFamily::Switches, 0x00) => self.touch_lock = D1203TouchLock(value),
            (SwitchFamily::Switches, 0x03) => self.multipoint = D1203Multipoint(value),
            (SwitchFamily::Switches, 0x08) => self.wear_detection = D1203WearDetection(value),
            (SwitchFamily::Switches, 0x1d) => self.soundstage = D1203Soundstage(value),
            _ => {}
        }
    }

    /// The EQ preset reported by the device (read-only/informational).
    #[allow(dead_code)]
    pub fn preset_equalizer(&self) -> D1203PresetEqualizer {
        self.preset_equalizer
    }
}

impl From<D1203StateUpdatePacket> for D1203State {
    fn from(value: D1203StateUpdatePacket) -> Self {
        Self {
            dual_battery: value.dual_battery,
            dual_firmware_version: value.dual_firmware_version,
            serial_number: value.serial_number,
            preset_equalizer: value.preset_equalizer,
            custom_equalizer: value.custom_equalizer,
            ambient_sound_mode: value.ambient_sound_mode,
            nc_mode: value.nc_mode,
            manual_nc_level: value.manual_nc_level,
            transportation_mode: value.transportation_mode,
            wind_noise: value.wind_noise,
            button_configuration: value.button_configuration,
            touch_tone: value.touch_tone,
            low_battery_prompt: value.low_battery_prompt,
            wearing_tone: value.wearing_tone,
            ldac: value.ldac,
            easy_chat: value.easy_chat,
            soundstage: value.soundstage,
            auto_power_off: value.auto_power_off,
            case_battery_level: value.case_battery_level,
            touch_lock: Default::default(),
            multipoint: Default::default(),
            wear_detection: Default::default(),
            touch_sensitivity: value.touch_sensitivity,
        }
    }
}

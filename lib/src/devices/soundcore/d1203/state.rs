use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    d1203::{
        modules::{
            D1203AmbientSoundMode, D1203AutoPowerOff, D1203ButtonConfiguration,
            D1203CustomEqualizer, D1203EasyChat, D1203Ldac, D1203LowBatteryPrompt,
            D1203ManualNoiseCanceling, D1203NoiseCancelingMode, D1203Soundstage, D1203TouchTone,
            D1203TransportationMode, D1203WearingTone, D1203WindNoise,
        },
        packets::inbound::{D1203PresetEqualizer, D1203StateUpdatePacket},
    },
    common::structures::{DualBattery, DualFirmwareVersion, SerialNumber},
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
    ///
    /// TODO(read): which TLV tag of the D1203 state packet carries the current
    /// ambient sound mode is not yet known (a full state capture is pending),
    /// so this is initialized to `Default` on connect. Writing works
    /// regardless; only reading back the device's *current* mode is a TODO.
    ambient_sound_mode: D1203AmbientSoundMode,
    /// Noise-canceling sub-mode (byte 1 of the `06 81` payload): Manual or
    /// Transportation. Only meaningful when the ambient mode is NoiseCanceling,
    /// but the device always carries a value here.
    ///
    /// TODO(read): like `ambient_sound_mode`, the state-packet TLV tag for this
    /// is not yet identified, so it is initialized to `Default` (Manual) on
    /// connect. Writing works regardless.
    nc_mode: D1203NoiseCancelingMode,
    /// Manual ANC level (byte 3 of the `06 81` payload), 1..=5.
    ///
    /// TODO(read): not yet decoded from the state packet (TLV tag unknown), so
    /// initialized to `Default` (level 1) on connect. Writing works regardless.
    manual_nc_level: D1203ManualNoiseCanceling,
    /// Transport type (Airplane / Car) that drives byte[5] of the `06 81`
    /// payload WHILE the NC sub-mode is Transportation. Outside Transportation
    /// byte[5] is the active flag and this field is ignored; see
    /// `modules::ambient_sound_mode::build_sound_mode_payload`.
    ///
    /// TODO(read): the D1203 state packet does not expose the current transport
    /// type (no TLV tag identified), so it is initialized to `Default`
    /// (Airplane) on connect. Writing works regardless.
    transportation_mode: D1203TransportationMode,
    /// Wind noise reduction toggle. Driven by its OWN dedicated command
    /// (`Command([0x41, 0x88])`, payload `[0x0b, 0|1]`), NOT the `06 81`
    /// sound-mode command. See `modules::wind_noise`.
    ///
    /// TODO(read): the state-packet TLV tag carrying this is not yet
    /// identified, so it is initialized to `Default` (OFF) on connect. Writing
    /// works regardless.
    wind_noise: D1203WindNoise,
    /// Current action of each of the eight configurable buttons (2 sides x 4
    /// gestures). Driven by the `04 8c` command, one packet per changed gesture.
    /// See `modules::button_configuration`.
    ///
    /// TODO(read): the D1203 state packet does not expose button configuration
    /// in any known form (state read-back for the D1203 has been investigated
    /// and is infeasible for most fields), so this is initialized to `Default`
    /// (every button = PlayPause) on connect. Writing works regardless.
    button_configuration: D1203ButtonConfiguration,
    /// Touch Tone prompt toggle. Driven by command `41 87` sub-id 0. See
    /// `modules::misc`.
    ///
    /// TODO(read): state-packet TLV tag not yet identified; initialized to
    /// `Default` (OFF). Writing works regardless.
    touch_tone: D1203TouchTone,
    /// Low Battery Prompt toggle. Driven by command `41 87` sub-id 1.
    ///
    /// TODO(read): state-packet TLV tag not yet identified; initialized to
    /// `Default` (OFF). Writing works regardless.
    low_battery_prompt: D1203LowBatteryPrompt,
    /// Wearing Tone prompt toggle. Driven by command `41 87` sub-id 3.
    ///
    /// TODO(read): state-packet TLV tag not yet identified; initialized to
    /// `Default` (OFF). Writing works regardless.
    wearing_tone: D1203WearingTone,
    /// LDAC high-resolution codec toggle. Driven by command `41 88` sub-id 2.
    ///
    /// TODO(read): state-packet TLV tag not yet identified; initialized to
    /// `Default` (OFF). Writing works regardless.
    ldac: D1203Ldac,
    /// Easy Chat toggle. Driven by command `41 88` sub-id 14.
    ///
    /// TODO(read): state-packet TLV tag not yet identified; initialized to
    /// `Default` (OFF). Writing works regardless.
    easy_chat: D1203EasyChat,
    /// Soundstage / Soundfield rendering toggle. Driven by command `41 88`
    /// sub-id 29. Exposed via `SettingId::SpatialAudio`.
    ///
    /// TODO(read): state-packet TLV tag not yet identified; initialized to
    /// `Default` (OFF). Writing works regardless.
    soundstage: D1203Soundstage,
    /// Auto power off timeout selector. Driven by command `01 86`, payload
    /// `[enabled, duration_idx]`. See `modules::misc`.
    ///
    /// TODO(read): state-packet TLV tag not yet identified; initialized to
    /// `Default` (Never). Writing works regardless.
    auto_power_off: D1203AutoPowerOff,
}

impl D1203State {
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
            // The device does not report the current custom parametric EQ in
            // its state packet, so start flat; the value tracks what we write.
            custom_equalizer: D1203CustomEqualizer::default(),
            // TODO: read the real current ambient sound mode from the state
            // packet once the TLV tag is identified. For now start at Default;
            // writes still apply correctly (the device accepts the 06 81
            // command, including value 3 = Adaptive).
            ambient_sound_mode: value.ambient_sound_mode,
            // TODO: read the real NC sub-mode and manual level from the state
            // packet once their TLV tags are identified. For now start at
            // Default; writes still apply correctly.
            nc_mode: value.nc_mode,
            manual_nc_level: value.manual_nc_level,
            // TODO: read the real transport type from the state packet once its
            // TLV tag is identified. For now start at Default (Airplane); writes
            // still apply correctly (byte[5] inside Transportation).
            transportation_mode: value.transportation_mode,
            // TODO: read the real wind-noise state from the state packet once
            // its TLV tag is identified. For now start at Default (OFF); writes
            // still apply correctly (the device accepts the 41 88 command).
            wind_noise: value.wind_noise,
            // TODO: the D1203 state packet does not expose button configuration
            // in any known form, so start at Default (every button = PlayPause)
            // on connect; writes still apply correctly (the device accepts the
            // 04 8c command, one packet per changed gesture).
            button_configuration: D1203ButtonConfiguration::default(),
            // TODO: read the real miscellaneous toggle / auto power off state
            // from the state packet once their TLV tags are identified. For now
            // start at Default; writes still apply correctly (each uses its own
            // dedicated command: 41 87 / 41 88 / 01 86).
            touch_tone: value.touch_tone,
            low_battery_prompt: value.low_battery_prompt,
            wearing_tone: value.wearing_tone,
            ldac: value.ldac,
            easy_chat: value.easy_chat,
            soundstage: value.soundstage,
            auto_power_off: value.auto_power_off,
        }
    }
}

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::{
    modules::sound_modes_v2::ToPacketBody,
    packet::{self, inbound::FromPacketBody, parsing::take_bool},
    structures::{AmbientSoundMode, Flag, WindNoise},
};

// [0x06,0x81] outbound (A3953CmdService.G1) / A3953AnalysisService.T2 inbound; anc_option_manual
// clamped 1-6, trans_option 0-2 (write only), other fields raw; wind_noise.is_detected never sent
// back. The official app only shows 3 ambient sound modes for this device (Normal/Transparency/
// Noise Canceling), no adjustable ANC strength, confirmed by direct inspection on real hardware
// (2026-09-14), so anc_option_manual/anc_option_auto/anc_automation_mode/anc_auto_sensitivity_level
// and trans_option are preserved on write but deliberately not exposed as settings.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub anc_option_manual: u8,
    pub anc_option_auto: u8,
    pub trans_option: u8,
    pub anc_automation_mode: u8,
    pub wind_noise: WindNoise,
    pub anc_auto_sensitivity_level: u8,
}

impl SoundModes {
    pub fn bytes(&self) -> [u8; 6] {
        [
            self.ambient_sound_mode as u8,
            (self.anc_option_manual.clamp(1, 6) << 4) | (self.anc_option_auto & 0x0F),
            self.trans_option.clamp(0, 2),
            self.anc_automation_mode,
            u8::from(self.wind_noise.is_suppression_enabled),
            self.anc_auto_sensitivity_level,
        ]
    }
}

impl FromPacketBody for SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3953 sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    le_u8,
                    le_u8,
                    le_u8,
                    WindNoise::take,
                    le_u8,
                ),
                |(
                    ambient_sound_mode,
                    manual_auto_byte,
                    trans_option,
                    anc_automation_mode,
                    wind_noise,
                    anc_auto_sensitivity_level,
                )| Self {
                    ambient_sound_mode,
                    anc_option_manual: (manual_auto_byte >> 4).clamp(1, 6),
                    anc_option_auto: manual_auto_byte & 0x0F,
                    trans_option,
                    anc_automation_mode,
                    wind_noise,
                    anc_auto_sensitivity_level,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacketBody for SoundModes {
    fn bytes(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }
}

// bArr[113..129] (A3953AnalysisService.R0/G0) wired into the shared
// common::modules::button_configuration machinery instead of a hand-rolled read-only struct.
// button_id per press kind and the TwsLowBits nibble packing were confirmed byte-for-byte on real
// hardware (2026-09-14). See a3953.rs's BUTTON_CONFIGURATION_SETTINGS for the write confirmation.

// [0x04,0x85] (Cmm2CmdData.L0/CmmBtCmdService.O); 0-4, no known display labels (string table wasn't decoded)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PressSensitivity(pub u8);

impl PressSensitivity {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |value: u8| Self(value.min(4))).parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 1] {
        [self.0]
    }
}

// A3953SpatialAudioVM.SPATIAL_MODE_FIXED/HEAD_TRACKING = 1/2
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    ::strum::FromRepr,
    ::strum::Display,
    ::strum::IntoStaticStr,
    ::strum::EnumString,
    ::strum::EnumIter,
    ::strum::VariantArray,
    ::openscq30_i18n_macros::Translate,
)]
#[repr(u8)]
pub enum SpatialMode {
    #[default]
    Fixed = 1,
    HeadTracking = 2,
}

impl SpatialMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self::from_repr(b).unwrap_or_default()).parse_complete(input)
    }
}

// A3953SpatialAudioVM.SOUND_MODE_MUSIC/MOVIE = 0/1
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    ::strum::FromRepr,
    ::strum::Display,
    ::strum::IntoStaticStr,
    ::strum::EnumString,
    ::strum::EnumIter,
    ::strum::VariantArray,
    ::openscq30_i18n_macros::Translate,
)]
#[repr(u8)]
pub enum SpatialContentMode {
    #[default]
    Music = 0,
    Movie = 1,
}

impl SpatialContentMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self::from_repr(b).unwrap_or_default()).parse_complete(input)
    }
}

// [0x10,0x81] (Cmm2CmdData.u2/CmmBtCmdService.b4); byte order matches A3953AnalysisService.R0's read order
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SpatialAudio {
    pub is_enabled: bool,
    pub effect_mode: SpatialMode,
    pub sound_mode: SpatialContentMode,
}

impl SpatialAudio {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3953 spatial audio",
            map(
                (take_bool, SpatialMode::take, SpatialContentMode::take),
                |(is_enabled, effect_mode, sound_mode)| Self {
                    is_enabled,
                    effect_mode,
                    sound_mode,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 3] {
        [
            u8::from(self.is_enabled),
            self.effect_mode as u8,
            self.sound_mode as u8,
        ]
    }
}

// [0x10,0x83] (Cmm2CmdData.w2/CmmBtCmdService.e1)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AmbientSoundPrompt(pub bool);

impl AmbientSoundPrompt {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(take_bool, Self).parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 1] {
        [u8::from(self.0)]
    }
}

impl Flag for AmbientSoundPrompt {
    fn get_bool(&self) -> bool {
        self.0
    }

    fn set_bool(&mut self, value: bool) {
        self.0 = value;
    }
}

// bArr[63] (A3953AnalysisService.R0's m3); 255 or 254 = uninitialized (unlike a3955, which only
// checks one sentinel). Determines whether the equalizer write path sends the "uninitialized"
// sentinel bytes or the real hear id already stored on the device.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct IsHearIdInitialized(pub bool);

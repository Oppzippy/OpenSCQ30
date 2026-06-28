use std::fmt::Write;

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use strum::{Display, EnumIter, EnumString, FromRepr, IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::common::{
        self,
        modules::sound_modes_v2::ToPacketBody,
        packet::{self, inbound::FromPacketBody, parsing::take_bool},
    },
    i18n::fl,
};
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CaseFirmwareVersion(pub common::structures::FirmwareVersion);

impl CaseFirmwareVersion {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "case firmware version",
            map(common::structures::FirmwareVersion::take, Self),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        self.0.bytes().into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseSerialNumber(String);

impl Default for CaseSerialNumber {
    fn default() -> Self {
        Self("3954000000000000".to_string())
    }
}

impl std::fmt::Display for CaseSerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl CaseSerialNumber {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "case serial number",
            map(take(6usize), |bytes: &[u8]| {
                let mut serial_number = String::with_capacity(16);
                serial_number.push_str("3954");
                for b in bytes {
                    write!(serial_number, "{b:02X}")
                        .expect("we're writing to a string, so it should be infallible");
                }
                Self(serial_number)
            }),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        let num = u64::from_str_radix(&self.0, 16).unwrap();
        num.to_be_bytes().into_iter().skip(2)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub sound_mode_slider: u8,
    pub airplane_mode: AirplaneMode,
    pub wind_noise: WindNoise,
}

impl SoundModes {
    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [
            self.ambient_sound_mode as u8,
            self.sound_mode_slider,
            self.airplane_mode as u8,
            self.wind_noise.byte(),
        ]
        .into_iter()
    }
}

impl FromPacketBody for SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    le_u8,
                    AirplaneMode::take,
                    WindNoise::take,
                ),
                |(ambient_sound_mode, sound_mode_slider, airplane_mode, wind_noise)| Self {
                    ambient_sound_mode,
                    sound_mode_slider,
                    airplane_mode,
                    wind_noise,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacketBody for SoundModes {
    fn bytes(&self) -> Vec<u8> {
        self.bytes().collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct WindNoise {
    pub is_suppression_enabled: bool,
    pub is_detected: bool,
}

impl WindNoise {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "wind noise",
            map(le_u8, |wind_noise| Self {
                is_suppression_enabled: wind_noise & 1 != 0,
                is_detected: wind_noise & 2 != 0,
            }),
        )
        .parse_complete(input)
    }

    pub fn byte(&self) -> u8 {
        u8::from(self.is_suppression_enabled) | (u8::from(self.is_detected) << 1)
    }
}

#[repr(u8)]
#[derive(
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Display,
    Default,
    IntoStaticStr,
    EnumString,
    EnumIter,
    VariantArray,
    Translate,
)]
pub enum AmbientSoundMode {
    #[default]
    NoiseCanceling = 0,
    Transparency = 1,
    Normal = 2,
    AirplaneMode = 3,
}

impl AmbientSoundMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self::from_repr(b).unwrap_or_default()).parse_complete(input)
    }
}

#[derive(
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Display,
    Default,
    IntoStaticStr,
    EnumString,
    EnumIter,
    VariantArray,
    Translate,
)]
#[repr(u8)]
pub enum AirplaneMode {
    #[default]
    ManualUpdate = 0,
    AutomaticUpdate = 1,
}

impl AirplaneMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self::from_repr(b).unwrap_or_default()).parse_complete(input)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaseFeatures {
    pub is_atmospheric_enabled: bool,
    pub is_remote_camera_enabled: bool,
    pub is_find_device_enabled: bool,
    pub is_spatial_audio_enabled: bool,
}

impl CaseFeatures {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(
            (take_bool, take_bool, take_bool, take_bool),
            |(
                is_atmospheric_enabled,
                is_remote_camera_enabled,
                is_find_device_enabled,
                is_spatial_audio_enabled,
            )| Self {
                is_atmospheric_enabled,
                is_remote_camera_enabled,
                is_find_device_enabled,
                is_spatial_audio_enabled,
            },
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [
            u8::from(self.is_atmospheric_enabled),
            u8::from(self.is_remote_camera_enabled),
            u8::from(self.is_find_device_enabled),
            u8::from(self.is_spatial_audio_enabled),
        ]
        .into_iter()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AirPressure(u8);

impl AirPressure {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, Self).parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        std::iter::once(self.0)
    }
}

impl std::fmt::Display for AirPressure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0 / 100, self.0 % 100)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpatialAudio {
    pub is_enabled: bool,
    pub mode: SpatialAudioMode,
    pub music_mode: SpatialAudioMusicMode,
}

impl SpatialAudio {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "spatial audio",
            map(
                (
                    take_bool,
                    SpatialAudioMusicMode::take,
                    SpatialAudioMode::take,
                ),
                |(is_enabled, music_mode, mode)| Self {
                    is_enabled,
                    mode,
                    music_mode,
                },
            ),
        )
        .parse_complete(input)
    }
    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [
            u8::from(self.is_enabled),
            self.music_mode as u8,
            self.mode as u8,
        ]
        .into_iter()
    }
}

#[derive(
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Display,
    Default,
    IntoStaticStr,
    EnumString,
    EnumIter,
    VariantArray,
    Translate,
)]
#[repr(u8)]
pub enum SpatialAudioMode {
    #[default]
    Music = 0,
    Podcast = 1,
    Movie = 2,
    Gaming = 3,
}

impl SpatialAudioMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self::from_repr(b).unwrap_or_default()).parse_complete(input)
    }
}

#[derive(
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Display,
    Default,
    IntoStaticStr,
    EnumString,
    EnumIter,
    VariantArray,
    Translate,
)]
#[repr(u8)]
pub enum SpatialAudioMusicMode {
    #[default]
    Fixed = 1,
    HeadTracking = 2,
}

impl SpatialAudioMusicMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self::from_repr(b).unwrap_or_default()).parse_complete(input)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct EasyChat {
    pub is_enabled: bool,
    pub wait_time: EasyChatWaitTime,
}

impl EasyChat {
    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        std::iter::once(u8::from(self.is_enabled)).chain(self.wait_time.bytes())
    }
}

#[derive(
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Display,
    Default,
    IntoStaticStr,
    EnumString,
    EnumIter,
    VariantArray,
)]
#[repr(u8)]
pub enum EasyChatWaitTime {
    #[default]
    #[strum(serialize = "5s")]
    FiveSeconds = 0,
    #[strum(serialize = "10s")]
    TenSeconds = 1,
    #[strum(serialize = "15s")]
    FifteenSeconds = 2,
}

impl openscq30_i18n::Translate for EasyChatWaitTime {
    fn translate(&self) -> String {
        match self {
            EasyChatWaitTime::FiveSeconds => fl!("x-seconds", seconds = 5),
            EasyChatWaitTime::TenSeconds => fl!("x-seconds", seconds = 10),
            EasyChatWaitTime::FifteenSeconds => fl!("x-seconds", seconds = 15),
        }
    }
}

impl EasyChatWaitTime {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self::from_repr(b).unwrap_or_default()).parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        std::iter::once(*self as u8)
    }
}

#[derive(
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Display,
    Default,
    IntoStaticStr,
    EnumString,
    EnumIter,
    VariantArray,
)]
#[repr(u8)]
pub enum CaseLanguage {
    #[default]
    Japanese = 0,
    English = 1,
    Chinese = 2,
}

impl openscq30_i18n::Translate for CaseLanguage {
    fn translate(&self) -> String {
        // TODO do something slimilar to openscq30-gui for using fl! to translate language names
        match self {
            CaseLanguage::Japanese => "日本語".to_string(),
            CaseLanguage::English => "English".to_string(),
            CaseLanguage::Chinese => "中文".into(),
        }
    }
}

impl CaseLanguage {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self::from_repr(b).unwrap_or_default()).parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        std::iter::once(*self as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_serial_number() {
        let sn = CaseSerialNumber("3954123456789ABC".to_string());
        assert_eq!(
            sn.bytes().collect::<Vec<u8>>(),
            vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]
        );
    }
}

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr, VariantArray};

use crate::devices::soundcore::common::{
    macros::sound_mode_enum,
    modules::sound_modes_v2::ToPacketBody,
    packet::{self, inbound::FromPacketBody, parsing::take_bool},
    structures::AmbientSoundMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub manual_noise_canceling: ManualNoiseCanceling,
    pub adaptive_noise_canceling: AdaptiveNoiseCanceling,
    pub transparency_mode: TransparencyMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub wind_noise: WindNoise,
    pub multi_scene_anc: MultiSceneAnc,
    pub real_time_adaptive_anc: bool,
}

impl SoundModes {
    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [
            self.ambient_sound_mode.id(),
            (self.manual_noise_canceling.inner() << 4) | (self.adaptive_noise_canceling as u8),
            self.transparency_mode.byte(),
            self.noise_canceling_mode.byte(),
            self.wind_noise.byte(),
            0,
            self.multi_scene_anc.byte(),
            u8::from(self.real_time_adaptive_anc),
        ]
        .into_iter()
    }
}

impl ToPacketBody for SoundModes {
    fn bytes(&self) -> Vec<u8> {
        self.bytes().collect()
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
                    take_noise_canceling_strength,
                    TransparencyMode::take,
                    NoiseCancelingMode::take,
                    WindNoise::take,
                    le_u8,
                    MultiSceneAnc::take,
                    take_bool,
                ),
                |(
                    ambient_sound_mode,
                    (manual_noise_canceling, adaptive_noise_canceling),
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise,
                    _unknown,
                    multi_scene_anc,
                    real_time_adaptive_anc,
                )| Self {
                    ambient_sound_mode,
                    transparency_mode,
                    noise_canceling_mode,
                    multi_scene_anc,
                    real_time_adaptive_anc,
                    manual_noise_canceling,
                    adaptive_noise_canceling,
                    wind_noise,
                },
            ),
        )
        .parse_complete(input)
    }
}

sound_mode_enum!(
    pub enum TransparencyMode {
        FullyTransparent = 0,
        VocalMode = 1,
    }
);

sound_mode_enum!(
    pub enum NoiseCancelingMode {
        Manual = 0,
        Adaptive = 1,
        MultiScene = 2,
    }
);

sound_mode_enum!(
    pub enum MultiSceneAnc {
        Transport = 0,
        Outdoor = 1,
        Indoor = 2,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManualNoiseCanceling(u8);

impl Default for ManualNoiseCanceling {
    fn default() -> Self {
        Self::new(1)
    }
}

impl ManualNoiseCanceling {
    pub fn new(value: u8) -> Self {
        Self(value.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    FromRepr,
    IntoStaticStr,
    EnumString,
    EnumIter,
    VariantArray,
    Translate,
)]
#[repr(u8)]
pub enum AdaptiveNoiseCanceling {
    #[default]
    Weak = 0,
    Moderate = 1,
    Strong = 2,
}

fn take_noise_canceling_strength<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], (ManualNoiseCanceling, AdaptiveNoiseCanceling), E> {
    map(le_u8, |b| {
        (
            ManualNoiseCanceling::new((b & 0xF0) >> 4),
            AdaptiveNoiseCanceling::from_repr(b & 0x0F).unwrap_or_default(),
        )
    })
    .parse_complete(input)
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpatialAudio {
    pub is_enabled: bool,
    pub mode: SpatialAudioMode,
}

impl SpatialAudio {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "spatial audio",
            map((take_bool, SpatialAudioMode::take), |(is_enabled, mode)| {
                Self { is_enabled, mode }
            }),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [u8::from(self.is_enabled), self.mode as u8].into_iter()
    }
}

sound_mode_enum!(
    pub enum SpatialAudioMode {
        Music = 0,
        Movie = 2,
        Gaming = 3,
    }
);

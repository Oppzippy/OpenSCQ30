use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use strum::{Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::devices::soundcore::standard::structures::{
    AmbientSoundMode, NoiseCancelingMode, TransparencyMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct A3959SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub transparency_mode: TransparencyMode,
    pub adaptive_noise_canceling: AdaptiveNoiseCanceling,
    pub manual_noise_canceling: ManualNoiseCanceling,
    pub noise_canceling_mode: A3959NoiseCancelingMode,
    pub wind_noise_suppression: bool,
    pub noise_canceling_adaptive_sensitivity_level: u8,
    pub multi_scene_anc: NoiseCancelingMode,
}

impl A3959SoundModes {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3959SoundModes, E> {
        context(
            "a3959 sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    NoiseCancelingSettings::take,
                    TransparencyMode::take,
                    A3959NoiseCancelingMode::take,
                    WindNoise::take,
                    le_u8,
                    NoiseCancelingMode::take,
                ),
                |(
                    ambient_sound_mode,
                    noise_canceling_settings,
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise,
                    noise_canceling_adaptive_sensitivity_level,
                    multi_scene_anc,
                )| {
                    A3959SoundModes {
                        ambient_sound_mode,
                        transparency_mode,
                        adaptive_noise_canceling: noise_canceling_settings.adaptive,
                        manual_noise_canceling: noise_canceling_settings.manual,
                        noise_canceling_mode,
                        wind_noise_suppression: wind_noise.is_suppression_enabled,
                        noise_canceling_adaptive_sensitivity_level,
                        multi_scene_anc,
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    pub(crate) fn bytes(&self) -> [u8; 7] {
        [
            self.ambient_sound_mode.id(),
            (self.manual_noise_canceling.0 << 4) | self.adaptive_noise_canceling.inner(),
            self.transparency_mode.id(),
            self.noise_canceling_mode.id(), // ANC automation mode?
            self.wind_noise_suppression.into(),
            self.noise_canceling_adaptive_sensitivity_level,
            self.multi_scene_anc.id(),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct AdaptiveNoiseCanceling(u8);

impl AdaptiveNoiseCanceling {
    pub fn new(value: u8) -> Self {
        Self(value.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ManualNoiseCanceling(u8);

impl ManualNoiseCanceling {
    pub fn new(value: u8) -> Self {
        Self(value.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

struct NoiseCancelingSettings {
    manual: ManualNoiseCanceling,
    adaptive: AdaptiveNoiseCanceling,
}

impl NoiseCancelingSettings {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], NoiseCancelingSettings, E> {
        map(le_u8, |b| NoiseCancelingSettings {
            manual: ManualNoiseCanceling::new((b & 0xF0) >> 4),
            adaptive: AdaptiveNoiseCanceling::new(b & 0x0F),
        })
        .parse_complete(input)
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
    Translate,
)]
pub enum A3959NoiseCancelingMode {
    #[default]
    Adaptive = 0,
    Manual = 1,
    MultiScene = 2,
}

impl A3959NoiseCancelingMode {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3959NoiseCancelingMode, E> {
        context(
            "a3959 noise canceling mode",
            map(le_u8, |noise_canceling_mode| {
                A3959NoiseCancelingMode::from_repr(noise_canceling_mode).unwrap_or_default()
            }),
        )
        .parse_complete(input)
    }
}

impl A3959NoiseCancelingMode {
    pub fn id(&self) -> u8 {
        *self as u8
    }
}

pub struct WindNoise {
    pub is_suppression_enabled: bool,
    pub is_detected: bool,
}

impl WindNoise {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], WindNoise, E> {
        context(
            "wind noise",
            map(le_u8, |wind_noise| WindNoise {
                is_suppression_enabled: wind_noise & 1 != 0,
                is_detected: wind_noise & 2 != 0,
            }),
        )
        .parse_complete(input)
    }
}

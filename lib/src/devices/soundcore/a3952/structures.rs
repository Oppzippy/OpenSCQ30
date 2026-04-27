use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use openscq30_lib_macros::MigrationSteps;
use strum::{Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::devices::soundcore::common::{self, modules::sound_modes_v2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, MigrationSteps)]
pub struct SoundModes {
    pub ambient_sound_mode: common::structures::AmbientSoundMode,
    #[migration_requirement(field = ambient_sound_mode, value = common::structures::AmbientSoundMode::Transparency)]
    pub transparency_mode: common::structures::TransparencyMode,
    #[migration_requirement(field = ambient_sound_mode, value = common::structures::AmbientSoundMode::NoiseCanceling)]
    pub noise_canceling_mode: NoiseCancelingMode,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Manual)]
    pub manual_noise_canceling: ManualNoiseCanceling,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Adaptive)]
    pub adaptive_noise_canceling: AdaptiveNoiseCanceling,
    #[migration_requirement(
        field = ambient_sound_mode,
        value = common::structures::AmbientSoundMode::NoiseCanceling,
        value2 = common::structures::AmbientSoundMode::Transparency,
    )]
    pub wind_noise: WindNoise,
    pub unknown: u8,
}

impl SoundModes {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3952 sound modes",
            map(
                (
                    common::structures::AmbientSoundMode::take,
                    le_u8, // manual/adaptive noise canceling level
                    common::structures::TransparencyMode::take,
                    NoiseCancelingMode::take,
                    WindNoise::take,
                    le_u8, // unknown
                ),
                |(
                    ambient_sound_mode,
                    manual_adaptive_level,
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise,
                    unknown,
                )| {
                    Self {
                        ambient_sound_mode,
                        manual_noise_canceling: ManualNoiseCanceling::from_repr(
                            (manual_adaptive_level & 0xF0) >> 4,
                        )
                        .unwrap_or_default(),
                        adaptive_noise_canceling: AdaptiveNoiseCanceling::from_repr(
                            manual_adaptive_level & 0xF,
                        )
                        .unwrap_or_default(),
                        transparency_mode,
                        noise_canceling_mode,
                        wind_noise,
                        unknown,
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 6] {
        [
            self.ambient_sound_mode as u8,
            ((self.manual_noise_canceling as u8) << 4) | self.adaptive_noise_canceling as u8,
            self.transparency_mode as u8,
            self.noise_canceling_mode as u8,
            self.wind_noise.byte(),
            self.unknown,
        ]
    }
}

impl sound_modes_v2::ToPacketBody for SoundModes {
    fn bytes(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    FromRepr,
    Translate,
    EnumIter,
    IntoStaticStr,
    EnumString,
)]
#[repr(u8)]
pub enum NoiseCancelingMode {
    #[default]
    Manual = 0,
    Adaptive = 1,
}

impl NoiseCancelingMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self::from_repr(b).unwrap_or_default()).parse_complete(input)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    FromRepr,
    Translate,
    EnumIter,
    IntoStaticStr,
    EnumString,
)]
#[repr(u8)]
pub enum ManualNoiseCanceling {
    #[default]
    Weak = 1,
    Moderate = 2,
    Strong = 3,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    FromRepr,
    Translate,
    EnumIter,
    IntoStaticStr,
    Display,
    EnumString,
)]
#[repr(u8)]
#[allow(clippy::enum_variant_names)]
pub enum AdaptiveNoiseCanceling {
    #[default]
    LowNoise = 0,
    MediumNoise = 1,
    HighNoise = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use openscq30_lib_macros::MigrationSteps;
use strum::{Display, EnumIter, EnumString, FromRepr, IntoStaticStr, VariantArray};

use crate::devices::soundcore::common::{self, modules::sound_modes_v2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, MigrationSteps)]
pub struct SoundModes {
    pub ambient_sound_mode: common::structures::AmbientSoundMode,
    #[migration_requirement(field = ambient_sound_mode, value = common::structures::AmbientSoundMode::Transparency)]
    pub transparency_mode: common::structures::TransparencyMode,
    #[migration_requirement(field = ambient_sound_mode, value = common::structures::AmbientSoundMode::NoiseCanceling)]
    pub noise_canceling_mode: NoiseCancelingMode,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Adaptive)]
    pub adaptive_noise_canceling: AdaptiveNoiseCanceling,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Manual)]
    pub manual_noise_canceling: ManualNoiseCanceling,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Transportation)]
    pub transportation_mode: TransportationMode,
    #[migration_requirement(field = ambient_sound_mode, value = common::structures::AmbientSoundMode::NoiseCanceling)]
    pub wind_noise: WindNoise,
    #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Adaptive)]
    pub noise_canceling_adaptive_sensitivity_level: u8,
}

impl SoundModes {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3957 sound modes",
            map(
                (
                    common::structures::AmbientSoundMode::take,
                    NoiseCancelingSettings::take,
                    common::structures::TransparencyMode::take,
                    NoiseCancelingMode::take,
                    WindNoise::take,
                    le_u8,
                    TransportationMode::take,
                ),
                |(
                    ambient_sound_mode,
                    noise_canceling_settings,
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise,
                    noise_canceling_adaptive_sensitivity_level,
                    transportation_mode,
                )| {
                    Self {
                        ambient_sound_mode,
                        transparency_mode,
                        adaptive_noise_canceling: noise_canceling_settings.adaptive,
                        manual_noise_canceling: noise_canceling_settings.manual,
                        noise_canceling_mode,
                        wind_noise,
                        noise_canceling_adaptive_sensitivity_level,
                        transportation_mode,
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 7] {
        [
            self.ambient_sound_mode.id(),
            (self.manual_noise_canceling.0 << 4) | self.adaptive_noise_canceling as u8,
            self.transparency_mode.id(),
            self.noise_canceling_mode.id(),
            self.wind_noise.byte(),
            self.noise_canceling_adaptive_sensitivity_level,
            self.transportation_mode as u8,
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
    PartialOrd,
    Ord,
    FromRepr,
    Translate,
    IntoStaticStr,
)]
#[repr(u8)]
pub enum AdaptiveNoiseCanceling {
    #[default]
    Weak = 1,
    Moderate = 2,
    Strong = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ManualNoiseCanceling(pub(crate) u8);

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
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |b| Self {
            manual: ManualNoiseCanceling::new((b & 0xF0) >> 4),
            adaptive: AdaptiveNoiseCanceling::from_repr(b & 0x0F).unwrap_or_default(),
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
    VariantArray,
    Translate,
)]
pub enum NoiseCancelingMode {
    #[default]
    Manual = 0,
    Adaptive = 1,
    Transportation = 2,
}

impl NoiseCancelingMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3957 noise canceling mode",
            map(le_u8, |noise_canceling_mode| {
                Self::from_repr(noise_canceling_mode).unwrap_or_default()
            }),
        )
        .parse_complete(input)
    }
}

impl NoiseCancelingMode {
    pub fn id(&self) -> u8 {
        *self as u8
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
pub enum TransportationMode {
    #[default]
    Plane = 0,
    Car = 3,
}

impl TransportationMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3957 transportation mode",
            map(le_u8, |v| Self::from_repr(v).unwrap_or_default()),
        )
        .parse_complete(input)
    }
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

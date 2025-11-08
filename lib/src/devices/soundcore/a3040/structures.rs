use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::devices::soundcore::common::{packet::parsing::take_bool, structures::AmbientSoundMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub manual_noise_canceling: ManualNoiseCanceling,
    pub adaptive_noise_canceling: AdaptiveNoiseCanceling,
    pub transparency_mode: TransparencyMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub wind_noise_reduction: bool,
    pub manual_transparency: ManualTransparency,
}

impl SoundModes {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3040 sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    take_manual_and_adaptive_noise_canceling,
                    TransparencyMode::take,
                    NoiseCancelingMode::take,
                    take_bool,
                    ManualTransparency::take,
                ),
                |(
                    ambient_sound_mode,
                    (manual_noise_canceling, adaptive_noise_canceling),
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise_reduction,
                    manual_transparency,
                )| Self {
                    ambient_sound_mode,
                    manual_noise_canceling,
                    adaptive_noise_canceling,
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise_reduction,
                    manual_transparency,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 6] {
        [
            self.ambient_sound_mode.id(),
            (self.manual_noise_canceling.inner() << 4) | self.adaptive_noise_canceling.inner(),
            self.transparency_mode as u8,
            self.noise_canceling_mode as u8,
            self.wind_noise_reduction.into(),
            self.manual_transparency.0,
        ]
    }
}

#[repr(u8)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    FromRepr,
    EnumIter,
    Translate,
    EnumString,
    IntoStaticStr,
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum TransparencyMode {
    #[default]
    TalkMode = 0,
    Manual = 1,
}

impl TransparencyMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |id| {
            TransparencyMode::from_repr(id).unwrap_or_default()
        })
        .parse_complete(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManualTransparency(pub u8);

impl ManualTransparency {
    pub fn new(byte: u8) -> Self {
        Self(byte.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, Self).parse_complete(input)
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for ManualTransparency {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy;

        (1u8..=5u8).prop_map(Self::new)
    }

    type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
}

#[repr(u8)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    FromRepr,
    EnumIter,
    Translate,
    EnumString,
    IntoStaticStr,
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum NoiseCancelingMode {
    #[default]
    Manual = 0,
    Adaptive = 1,
}

impl NoiseCancelingMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |id| {
            NoiseCancelingMode::from_repr(id).unwrap_or_default()
        })
        .parse_complete(input)
    }
}

pub fn take_manual_and_adaptive_noise_canceling<
    'a,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
>(
    input: &'a [u8],
) -> IResult<&'a [u8], (ManualNoiseCanceling, AdaptiveNoiseCanceling), E> {
    map(le_u8, |b| {
        (
            ManualNoiseCanceling::new(b >> 4),
            AdaptiveNoiseCanceling::new(b & 0xF),
        )
    })
    .parse_complete(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManualNoiseCanceling(u8);

impl ManualNoiseCanceling {
    pub fn new(byte: u8) -> Self {
        Self(byte.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for ManualNoiseCanceling {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy;

        (1u8..=5u8).prop_map(Self::new)
    }

    type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AdaptiveNoiseCanceling(u8);

impl AdaptiveNoiseCanceling {
    pub fn new(byte: u8) -> Self {
        Self(byte.clamp(0, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for AdaptiveNoiseCanceling {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy;

        (0u8..=5u8).prop_map(Self::new)
    }

    type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DoublePressAction(pub u8);

impl DoublePressAction {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, Self).parse_complete(input)
    }
}

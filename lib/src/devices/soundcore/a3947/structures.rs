use std::iter;

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::{le_i32, le_u8},
};
use openscq30_i18n_macros::Translate;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::devices::soundcore::common::{
    packet::parsing::take_bool,
    structures::{
        AmbientSoundMode, HearIdMusicType, HearIdType, TransparencyMode, VolumeAdjustments,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub transparency_mode: TransparencyMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub transportation_mode: TransportationMode,
    pub manual_noise_canceling: ManualNoiseCanceling,
    pub adaptive_noise_canceling: AdaptiveNoiseCanceling,
    pub wind_noise: WindNoise,
    pub environment_detection: bool,
}

impl SoundModes {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3947 sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    NoiseCancelingSettings::take,
                    TransparencyMode::take,
                    NoiseCancelingMode::take,
                    WindNoise::take,
                    take_bool,
                    TransportationMode::take,
                ),
                |(
                    ambient_sound_mode,
                    noise_canceling_settings,
                    transparency_mode,
                    noise_canceling_mode,
                    wind_noise,
                    environment_detection,
                    transportation_mode,
                )| {
                    Self {
                        ambient_sound_mode,
                        transparency_mode,
                        noise_canceling_mode,
                        transportation_mode,
                        manual_noise_canceling: noise_canceling_settings.manual,
                        adaptive_noise_canceling: noise_canceling_settings.adaptive,
                        wind_noise,
                        environment_detection,
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 7] {
        [
            self.ambient_sound_mode as u8,
            NoiseCancelingSettings {
                adaptive: self.adaptive_noise_canceling,
                manual: self.manual_noise_canceling,
            }
            .byte(),
            self.transparency_mode as u8,
            self.noise_canceling_mode as u8,
            self.wind_noise.byte(),
            self.environment_detection.into(),
            self.transportation_mode as u8,
        ]
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
    EnumIter,
    Translate,
    IntoStaticStr,
    EnumString,
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[repr(u8)]
pub enum NoiseCancelingMode {
    #[default]
    Manual = 0,
    Adaptive = 1,
    Transportation = 2,
}

impl NoiseCancelingMode {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3947 noise canceling mode",
            map(le_u8, |noise_canceling_mode| {
                Self::from_repr(noise_canceling_mode).unwrap_or_default()
            }),
        )
        .parse_complete(input)
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
    EnumIter,
    Translate,
    IntoStaticStr,
    EnumString,
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[repr(u8)]
pub enum TransportationMode {
    #[default]
    Plane = 0,
    Train = 1,
    Bus = 2,
    Car = 3,
}

impl TransportationMode {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3947 noise canceling mode",
            map(le_u8, |noise_canceling_mode| {
                Self::from_repr(noise_canceling_mode).unwrap_or_default()
            }),
        )
        .parse_complete(input)
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

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for AdaptiveNoiseCanceling {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy;

        (1u8..=5u8).prop_map(Self::new)
    }

    type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
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

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for ManualNoiseCanceling {
    type Parameters = ();

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::Strategy;

        (1u8..=5u8).prop_map(Self::new)
    }

    type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
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
            adaptive: AdaptiveNoiseCanceling::new(b & 0x0F),
        })
        .parse_complete(input)
    }

    fn byte(&self) -> u8 {
        self.manual.0 << 4 | self.adaptive.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct WindNoise {
    pub is_suppression_enabled: bool,
    pub is_detected: bool,
}

impl WindNoise {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HearId<const C: usize, const B: usize> {
    pub is_enabled: bool,
    pub volume_adjustments: [VolumeAdjustments<B>; C],
    pub time: i32,
    pub hear_id_type: HearIdType,
    pub music_type: HearIdMusicType,
    pub custom_volume_adjustments: [VolumeAdjustments<B>; C],
}

impl<const C: usize, const B: usize> Default for HearId<C, B> {
    fn default() -> Self {
        Self {
            is_enabled: Default::default(),
            volume_adjustments: [Default::default(); C],
            time: Default::default(),
            hear_id_type: Default::default(),
            music_type: Default::default(),
            custom_volume_adjustments: [Default::default(); C],
        }
    }
}

impl<const C: usize, const B: usize> HearId<C, B> {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3947 hear id",
            map(
                (
                    take_bool,
                    count(VolumeAdjustments::take, C),
                    le_i32,
                    HearIdType::take,
                    count(VolumeAdjustments::take, C),
                    HearIdMusicType::take,
                    le_u8,
                ),
                |(
                    is_enabled,
                    volume_adjustments,
                    time,
                    hear_id_type,
                    custom_volume_adjustments,
                    music_type,
                    _unknown,
                )| {
                    let volume_adjustments: [VolumeAdjustments<B>; C] = volume_adjustments
                        .try_into()
                        .expect("count is guaranteed to return a vec with the desired length");
                    let custom_volume_adjustments: [VolumeAdjustments<B>; C] =
                        custom_volume_adjustments
                            .try_into()
                            .expect("count is guaranteed to return a vec with the desired length");
                    Self {
                        is_enabled,
                        volume_adjustments,
                        time,
                        hear_id_type,
                        music_type,
                        custom_volume_adjustments,
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        iter::once(self.is_enabled.into())
            .chain(self.volume_adjustments.iter().flat_map(|side| side.bytes()))
            .chain(self.time.to_le_bytes())
            .chain([self.hear_id_type.0, 0])
            .chain(
                self.custom_volume_adjustments
                    .iter()
                    .flat_map(|side| side.bytes()),
            )
            .chain(iter::once(self.music_type.0))
    }
}

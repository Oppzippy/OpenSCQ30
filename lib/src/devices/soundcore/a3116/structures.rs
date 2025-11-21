use std::iter;

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::i18n::fl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Volume(pub u8);

impl Volume {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("volume", map(le_u8, Self)).parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        iter::once(self.0 as u8)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    EnumIter,
    EnumString,
    IntoStaticStr,
    FromRepr,
)]
#[repr(u8)]
pub enum AutoPowerOffDuration {
    #[default]
    FiveMinutes = 1,
    TenMinutes = 2,
    ThirtyMintes = 3,
    SixtyMinutes = 4,
}

impl AutoPowerOffDuration {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "auto power off duration",
            map(le_u8, |index| Self::from_repr(index).unwrap_or_default()),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        iter::once(*self as u8)
    }
}

impl openscq30_i18n::Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            AutoPowerOffDuration::FiveMinutes => fl!("x-minutes", minutes = 5),
            AutoPowerOffDuration::TenMinutes => fl!("x-minutes", minutes = 10),
            AutoPowerOffDuration::ThirtyMintes => fl!("x-minutes", minutes = 30),
            AutoPowerOffDuration::SixtyMinutes => fl!("x-minutes", minutes = 60),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct EqualizerConfiguration {
    preset: Option<EqualizerPreset>,
    volume_adjustments: VolumeAdjustments,
}

impl EqualizerConfiguration {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "equalizer configuration",
            map(
                (le_u8, VolumeAdjustments::take),
                |(preset_index, volume_adjustments)| Self {
                    preset: EqualizerPreset::from_repr(preset_index),
                    volume_adjustments,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        iter::once(self.preset.map_or(0xF, |preset| preset as u8))
            .chain(self.volume_adjustments.bytes())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Translate,
    FromRepr,
)]
#[repr(u8)]
pub enum EqualizerPreset {
    #[default]
    BassUp = 0,
    BassOff = 1,
    Voice = 2,
    Heavy = 3,
    Classic = 4,
    Original = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct VolumeAdjustments(pub [i8; 9]);

impl VolumeAdjustments {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(take(9usize), |bytes: &[u8]| {
            Self(
                bytes
                    .iter()
                    .copied()
                    .map(|b| {
                        let clamped_i8 = i8::try_from(b.clamp(0, 12)).expect(
                            "the range was clamped before converting, so this can not fail",
                        );
                        clamped_i8 - 6
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect(
                        "take is guaranteed to take 9 bytes, so it will always fit in a [i8; 9]",
                    ),
            )
        })
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 9] {
        self.0.map(|volume| {
            u8::try_from((volume + 6).clamp(0, 12))
                .expect("the range was clamped, so conversion can't fail")
        })
    }
}

use std::iter;

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::{
    devices::soundcore::common::structures::{EqualizerConfiguration, VolumeAdjustments},
    i18n::fl,
};

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

pub fn take_equalizer_configuration<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], EqualizerConfiguration<1, 9, -6, 6, 0>, E> {
    context(
        "equalizer configuration",
        map(
            (le_u8, VolumeAdjustments::take),
            |(preset_index, volume_adjustments)| {
                EqualizerConfiguration::new(preset_index.into(), [volume_adjustments])
            },
        ),
    )
    .parse_complete(input)
}

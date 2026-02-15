mod sound_modes;

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
pub use sound_modes::*;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::devices::soundcore::common;

common::structures::flag!(AncPersonalizedToEarCanal);

#[derive(
    Debug,
    Default,
    Eq,
    PartialEq,
    Clone,
    Copy,
    EnumIter,
    EnumString,
    IntoStaticStr,
    FromRepr,
    Translate,
)]
#[repr(u8)]
pub enum ImmersiveExperience {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

impl ImmersiveExperience {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "immersive experience",
            map(le_u8, |v| Self::from_repr(v).unwrap_or_default()),
        )
        .parse_complete(input)
    }
}

#[derive(
    Debug,
    Default,
    Eq,
    PartialEq,
    Clone,
    Copy,
    EnumIter,
    EnumString,
    IntoStaticStr,
    FromRepr,
    Translate,
)]
#[repr(u8)]
pub enum PressureSensitivity {
    Softest = 0,
    #[default]
    Medium = 1,
    Firmest = 2,
}

impl PressureSensitivity {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "pressure sensitivity",
            map(le_u8, |v| Self::from_repr(v).unwrap_or_default()),
        )
        .parse_complete(input)
    }
}

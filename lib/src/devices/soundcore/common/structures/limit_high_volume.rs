use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n::Translate;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::{devices::soundcore::common::packet::parsing::take_bool, i18n::fl};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LimitHighVolume {
    pub enabled: bool,
    pub db_limit: u8,
    pub refresh_rate: DecibelReadingRefreshRate,
}

impl Default for LimitHighVolume {
    fn default() -> Self {
        Self {
            enabled: false,
            db_limit: 80,
            refresh_rate: DecibelReadingRefreshRate::default(),
        }
    }
}

impl LimitHighVolume {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "limit high volume",
            map(
                (take_bool, le_u8, DecibelReadingRefreshRate::take),
                |(enabled, db_limit, refresh_rate)| Self {
                    enabled,
                    db_limit,
                    refresh_rate,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 3] {
        [self.enabled.into(), self.db_limit, self.refresh_rate as u8]
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
    Hash,
    Default,
    FromRepr,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
#[repr(u8)]
pub enum DecibelReadingRefreshRate {
    #[default]
    RealTime = 0,
    #[strum(serialize = "10s")]
    TenSeconds = 1,
    #[strum(serialize = "1m")]
    OneMinute = 2,
}

impl Translate for DecibelReadingRefreshRate {
    fn translate(&self) -> String {
        match self {
            Self::RealTime => fl!("real-time"),
            Self::TenSeconds => fl!("x-seconds", seconds = 10),
            Self::OneMinute => fl!("x-minutes", minutes = 1),
        }
    }
}

impl DecibelReadingRefreshRate {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "decibel reading refresh rate",
            map(le_u8, |refresh_rate| {
                Self::from_repr(refresh_rate).unwrap_or_default()
            }),
        )
        .parse_complete(input)
    }
}

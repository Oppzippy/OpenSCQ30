use std::iter;

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

const NOISE_CANCELING_MODE: u8 = 1 << 0;
const TRANSPARENCY_MODE: u8 = 1 << 1;
const NORMAL_MODE: u8 = 1 << 2;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AmbientSoundModeCycleTws {
    pub tws_enabled: AmbientSoundModeCycle,
    pub tws_disabled: AmbientSoundModeCycle,
}

impl AmbientSoundModeCycleTws {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("ambient sound mode cycle tws", map(le_u8, Self::from)).parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        iter::once((*self).into())
    }
}

impl From<u8> for AmbientSoundModeCycleTws {
    fn from(value: u8) -> Self {
        Self {
            tws_disabled: AmbientSoundModeCycle::from(value >> 4),
            tws_enabled: AmbientSoundModeCycle::from(value & 0xF),
        }
    }
}

impl From<AmbientSoundModeCycleTws> for u8 {
    fn from(value: AmbientSoundModeCycleTws) -> Self {
        (u8::from(value.tws_disabled) << 4) | u8::from(value.tws_enabled)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AmbientSoundModeCycle {
    pub noise_canceling_mode: bool,
    pub transparency_mode: bool,
    pub normal_mode: bool,
}

impl AmbientSoundModeCycle {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("ambient sound mode cycle", map(le_u8, Self::from)).parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 1] {
        [(*self).into()]
    }
}

impl Default for AmbientSoundModeCycle {
    fn default() -> Self {
        Self {
            noise_canceling_mode: true,
            transparency_mode: true,
            normal_mode: true,
        }
    }
}

impl From<u8> for AmbientSoundModeCycle {
    fn from(value: u8) -> Self {
        Self {
            noise_canceling_mode: value & NOISE_CANCELING_MODE != 0,
            transparency_mode: value & TRANSPARENCY_MODE != 0,
            normal_mode: value & NORMAL_MODE != 0,
        }
    }
}

impl From<AmbientSoundModeCycle> for u8 {
    fn from(value: AmbientSoundModeCycle) -> Self {
        [
            (value.noise_canceling_mode, NOISE_CANCELING_MODE),
            (value.transparency_mode, TRANSPARENCY_MODE),
            (value.normal_mode, NORMAL_MODE),
        ]
        .into_iter()
        .fold(
            0,
            |acc, (is_enabled, bit)| if is_enabled { acc | bit } else { acc },
        )
    }
}

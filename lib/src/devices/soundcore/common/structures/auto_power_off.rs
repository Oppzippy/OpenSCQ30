use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::packet::parsing::take_bool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AutoPowerOff {
    pub is_enabled: bool,
    pub duration: AutoPowerOffDurationIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AutoPowerOffDurationIndex(pub u8);

impl AutoPowerOff {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "auto power off",
            map((take_bool, le_u8), |(is_enabled, duration_index)| Self {
                is_enabled,
                duration: AutoPowerOffDurationIndex(duration_index),
            }),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 2] {
        [self.is_enabled.into(), self.duration.0]
    }
}

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::common::packet::parsing::take_bool;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct GamingMode {
    pub is_enabled: bool,
}

impl GamingMode {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "gaming mode",
            map(take_bool, |is_enabled| GamingMode { is_enabled }),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 1] {
        [self.is_enabled.into()]
    }
}

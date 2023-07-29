use nom::{
    combinator::map_opt,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::packets::structures::TransparencyMode;

use super::ParseResult;

pub fn take_transparency_mode<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<TransparencyMode, E> {
    context(
        "transparency mode",
        map_opt(le_u8, |transparency_mode| {
            TransparencyMode::from_id(transparency_mode)
        }),
    )(input)
}

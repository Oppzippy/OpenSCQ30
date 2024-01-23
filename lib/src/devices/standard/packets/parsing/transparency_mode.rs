use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::devices::standard::structures::TransparencyMode;

use super::ParseResult;

pub fn take_transparency_mode<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<TransparencyMode, E> {
    context(
        "transparency mode",
        map(le_u8, |transparency_mode| {
            TransparencyMode::from_id(transparency_mode).unwrap_or_default()
        }),
    )(input)
}

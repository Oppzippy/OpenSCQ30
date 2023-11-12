use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::devices::standard::structures::AgeRange;

use super::ParseResult;

pub fn take_age_range<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<AgeRange, E> {
    context("age range", map(le_u8, AgeRange))(input)
}

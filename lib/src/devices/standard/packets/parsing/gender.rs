use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::devices::standard::structures::Gender;

use super::ParseResult;

pub fn take_gender<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<Gender, E> {
    context("gender", map(le_u8, Gender))(input)
}

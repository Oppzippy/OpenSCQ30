use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::packets::structures::HearIdType;

use super::ParseResult;

pub fn take_hear_id_type<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<HearIdType, E> {
    context("hear id type", map(le_u8, HearIdType))(input)
}

use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
};

use crate::packets::structures::SerialNumber;

use super::{take_str, ParseResult};

pub fn take_serial_number<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<SerialNumber, E> {
    context("serial number", map(take_str(16usize), SerialNumber::from))(input)
}

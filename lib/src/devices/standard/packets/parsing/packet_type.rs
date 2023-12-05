use nom::{
    bytes::complete::take,
    combinator::map_opt,
    error::{context, ContextError, ParseError},
};

use super::ParseResult;

pub fn take_packet_type<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<[u8; 7], E> {
    context(
        "packet type 7 byte prefix",
        map_opt(take(7usize), |prefix: &[u8]| prefix.try_into().ok()),
    )(input)
}

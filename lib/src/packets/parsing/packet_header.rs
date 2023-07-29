use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u16,
    sequence::pair,
};

use crate::packets::structures::PacketHeader;

use super::{take_packet_type, ParseResult};

pub fn take_packet_header<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<PacketHeader, E> {
    context(
        "packet header",
        map(
            pair(take_packet_type, context("packet length", le_u16)),
            |(packet_type, length)| PacketHeader {
                packet_type,
                length,
            },
        ),
    )(input)
}

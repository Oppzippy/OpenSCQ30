use nom::{
    bytes::complete::take,
    combinator::{map, map_opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u16,
    sequence::pair,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::devices::standard::packets::parsing::ParseResult;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PacketHeader {
    pub packet_type: [u8; 7],
    pub length: u16,
}

impl PacketHeader {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
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
}

fn take_packet_type<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<[u8; 7], E> {
    context(
        "packet type 7 byte prefix",
        map_opt(take(7usize), |prefix: &[u8]| prefix.try_into().ok()),
    )(input)
}

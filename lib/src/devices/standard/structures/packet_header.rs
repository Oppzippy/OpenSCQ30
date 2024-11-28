use nom::{
    bytes::complete::take,
    combinator::{map, map_opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u16,
    sequence::pair,
    IResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Command;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PacketHeader {
    pub packet_type: Command,
    pub length: u16,
}

impl PacketHeader {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], PacketHeader, E> {
        context(
            "packet header",
            map(
                pair(take_command, context("packet length", le_u16)),
                |(packet_type, length)| PacketHeader {
                    packet_type,
                    length,
                },
            ),
        )(input)
    }
}

fn take_command<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], Command, E> {
    context(
        "packet type 7 byte prefix",
        map_opt(take(7usize), |prefix: &[u8]| {
            prefix.try_into().map(Command::new).ok()
        }),
    )(input)
}

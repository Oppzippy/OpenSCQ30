use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{map, map_opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u16,
    sequence::pair,
};

use super::Command;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
        )
        .parse_complete(input)
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
    )
    .parse_complete(input)
}

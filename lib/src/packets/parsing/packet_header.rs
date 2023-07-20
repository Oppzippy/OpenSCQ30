use nom::{combinator::map, error::context, number::complete::le_u16, sequence::pair};

use crate::packets::structures::PacketHeader;

use super::{take_packet_type, ParseResult};

pub fn take_packet_header(input: &[u8]) -> ParseResult<PacketHeader> {
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

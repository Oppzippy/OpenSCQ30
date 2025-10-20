mod checksum;
pub mod inbound;
mod multi_queue;
pub mod outbound;
mod packet_io_controller;
pub mod parsing;

pub use packet_io_controller::*;

use nom::{
    IResult, Parser,
    bytes::streaming::take,
    combinator::{map, map_opt},
    error::{ContextError, ParseError, context},
    number::streaming::{le_u8, le_u16},
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Packet {
    pub direction: Direction,
    pub command: Command,
    pub body: Vec<u8>,
}

impl Packet {
    pub fn bytes(&self) -> Vec<u8> {
        let direction_indicator = self.direction.bytes();
        let command = self.command.0;
        let body = &self.body;

        const PACKET_SIZE_LENGTH: usize = 2;
        const CHECKSUM_LENGTH: usize = 1;
        let length = direction_indicator.len()
            + command.len()
            + PACKET_SIZE_LENGTH
            + body.len()
            + CHECKSUM_LENGTH;

        let mut bytes = direction_indicator
            .into_iter()
            .chain(command)
            .chain((length as u16).to_le_bytes())
            .chain(body.iter().copied())
            .collect::<Vec<_>>();
        bytes.push(checksum::calculate_checksum(&bytes));

        bytes
    }

    /// This makes use of nom's streaming parsers, so Err::Incomplete will be returned if the packet
    /// is not done being read yet.
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        full_input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        let (input, (direction, command, length)) = context(
            "header",
            (
                Direction::take,
                Command::take,
                context("packet length", le_u16),
            ),
        )
        .parse_complete(full_input)?;
        let body_length = length.saturating_sub(10); // 5 byte direction, 2 byte command, 2 byte length, 1 byte checksum
        let (input, body) = context("body", take(body_length)).parse_complete(input)?;
        let header_and_body = &full_input[..full_input.len() - input.len()];
        let (input, _checksum) = context(
            "checksum",
            map_opt(le_u8, |checksum| {
                if checksum == checksum::calculate_checksum(header_and_body) {
                    Some(checksum)
                } else {
                    None
                }
            }),
        )
        .parse_complete(input)?;
        Ok((
            input,
            Self {
                direction,
                command,
                body: body.to_owned(),
            },
        ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub enum Direction {
    #[default]
    Outbound,
    Inbound,
}

impl Direction {
    pub const fn bytes(&self) -> [u8; 5] {
        match self {
            Self::Outbound => [0x08, 0xee, 0x00, 0x00, 0x00],
            Self::Inbound => [0x09, 0xff, 0x00, 0x00, 0x01],
        }
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "packet direction",
            map_opt(take(5usize), |direction_indicator: &[u8]| {
                if direction_indicator == Self::Inbound.bytes() {
                    Some(Self::Inbound)
                } else if direction_indicator == Self::Outbound.bytes() {
                    Some(Self::Outbound)
                } else {
                    None
                }
            }),
        )
        .parse_complete(input)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Command(pub [u8; 2]);

impl Command {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("command", map((le_u8, le_u8), |bytes| Self(bytes.into()))).parse_complete(input)
    }
}

pub mod checksum;
pub mod inbound;
pub mod multi_queue;
pub mod outbound;
pub mod packet_io_controller;
pub mod parsing;

use nom::{
    IResult, Parser,
    bytes::take,
    combinator::{map, map_opt},
    error::{ContextError, ParseError, context},
    number::complete::{le_u8, le_u16},
};

use crate::devices::soundcore::standard::packets::checksum::calculate_checksum;

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
            .chain(body.iter().cloned())
            .collect::<Vec<_>>();
        bytes.push(calculate_checksum(&bytes));

        bytes
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        let full_input = input;
        let (input, (direction, command, length)) = context(
            "header",
            (
                Direction::take,
                Command::take,
                context("packet length", le_u16),
            ),
        )
        .parse_complete(input)?;
        let body_length = length - 10; // 5 byte direction, 2 byte command, 2 byte length, 1 byte checksum
        let (input, body) = context("body", take(body_length)).parse_complete(input)?;
        let (input, _checksum) = context(
            "checksum",
            map_opt(le_u8, |checksum| {
                if checksum == calculate_checksum(&full_input[0..full_input.len() - 1]) {
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
            Direction::Outbound => [0x08, 0xee, 0x00, 0x00, 0x00],
            Direction::Inbound => [0x09, 0xff, 0x00, 0x00, 0x01],
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
        context(
            "command",
            map((le_u8, le_u8), |bytes| Command(bytes.into())),
        )
        .parse_complete(input)
    }
}

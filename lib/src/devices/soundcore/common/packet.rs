mod checksum;
pub mod inbound;
mod multi_queue;
pub mod outbound;
mod packet_io_controller;
pub mod parsing;

use std::marker::PhantomData;

pub use packet_io_controller::*;

use nom::{
    IResult, Parser,
    bytes::streaming::{tag, take},
    combinator::{map, map_opt},
    error::{ContextError, ParseError, context},
    number::streaming::{le_u8, le_u16},
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct InboundMarker;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct OutboundMarker;
pub trait HasDirection {
    const DIRECTION: Direction;
    type ReverseDirection: HasDirection;
}
impl HasDirection for InboundMarker {
    const DIRECTION: Direction = Direction::Inbound;
    type ReverseDirection = OutboundMarker;
}
impl HasDirection for OutboundMarker {
    const DIRECTION: Direction = Direction::Outbound;
    type ReverseDirection = InboundMarker;
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Packet<D> {
    pub command: Command,
    pub body: Vec<u8>,
    _d: PhantomData<D>,
}

impl<D> std::fmt::Debug for Packet<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Packet")
            .field("command", &self.command)
            .field("body", &self.body)
            .finish()
    }
}

pub type Inbound = Packet<InboundMarker>;
pub type Outbound = Packet<OutboundMarker>;

impl<D: HasDirection> Packet<D> {
    pub fn new(command: Command, body: Vec<u8>) -> Self {
        Self {
            command,
            body,
            _d: PhantomData,
        }
    }

    /// This makes use of nom's streaming parsers, so Err::Incomplete will be returned if the packet
    /// is not done being read yet.
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        checksum_kind: ChecksumKind,
    ) -> impl FnOnce(&'a [u8]) -> IResult<&'a [u8], Self, E> {
        // match makes sure we we have to update this function if another variant is added
        let has_checksum = match checksum_kind {
            ChecksumKind::None => false,
            ChecksumKind::Suffix => true,
        };
        move |full_input| {
            let (input, (_direction, command, length)) = context(
                "header",
                (
                    D::DIRECTION.take(),
                    Command::take,
                    context("packet length", le_u16),
                ),
            )
            .parse(full_input)?;

            // 5 byte direction, 2 byte command, 2 byte length, 1 byte checksum
            let body_length = length.saturating_sub(5 + 2 + 2 + if has_checksum { 1 } else { 0 });

            let (input, body) = context("body", take(body_length)).parse(input)?;
            let header_and_body = &full_input[..full_input.len() - input.len()];
            let input = if has_checksum {
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
                .parse(input)?;
                input
            } else {
                input
            };
            Ok((input, Self::new(command, body.to_vec())))
        }
    }

    /// Shorthand for Packet::take(ChecksumKind::Suffix)
    #[cfg(test)]
    pub fn take_with_checksum<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        Self::take(ChecksumKind::Suffix)(input)
    }

    /// Shorthand for Packet::take(ChecksumKind::None)
    #[cfg(test)]
    pub fn take_without_checksum<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        Self::take(ChecksumKind::None)(input)
    }

    pub fn bytes(&self, checksum_kind: ChecksumKind) -> Vec<u8> {
        // match makes sure we we have to update this function if another variant is added
        let has_checksum = match checksum_kind {
            ChecksumKind::None => false,
            ChecksumKind::Suffix => true,
        };

        const PACKET_SIZE_LENGTH: usize = 2;
        let checksum_length = if has_checksum { 1 } else { 0 };

        let direction_indicator = D::DIRECTION.bytes();
        let command = self.command.0;

        let length = direction_indicator.len()
            + command.len()
            + PACKET_SIZE_LENGTH
            + self.body.len()
            + checksum_length;

        let mut bytes = Vec::with_capacity(length);

        bytes.extend(
            direction_indicator
                .into_iter()
                .chain(command)
                .chain((length as u16).to_le_bytes())
                .chain(self.body.iter().copied()),
        );
        if has_checksum {
            bytes.push(checksum::calculate_checksum(&bytes));
        }

        bytes
    }

    /// Shorthand for .bytes(ChecksumKind::Suffix)
    #[cfg(test)]
    pub fn bytes_with_checksum(&self) -> Vec<u8> {
        self.bytes(ChecksumKind::Suffix)
    }

    /// Shorthand for .bytes(ChecksumKind::None)
    #[cfg(test)]
    pub fn bytes_without_checksum(&self) -> Vec<u8> {
        self.bytes(ChecksumKind::None)
    }

    pub fn ack(&self) -> Packet<D::ReverseDirection> {
        Packet::new(self.command, Vec::new())
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
        self,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E> {
        move |input| {
            context(
                "packet direction",
                map(tag(self.bytes().as_slice()), |_| self),
            )
            .parse(input)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Command(pub [u8; 2]);

impl Command {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("command", map((le_u8, le_u8), |bytes| Self(bytes.into()))).parse(input)
    }

    #[cfg(test)]
    pub fn ack<D: HasDirection>(self) -> Packet<D> {
        Packet::<D>::new(self, Vec::new())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ChecksumKind {
    None,
    #[default]
    Suffix,
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use super::*;

    #[test]
    fn to_and_from_bytes() {
        let packet = Outbound::new(Command([0, 1]), vec![2]);
        let packet_bytes = packet.bytes_with_checksum();
        let (remainder, parsed_packet) =
            Outbound::take_with_checksum::<VerboseError<_>>(&packet_bytes).unwrap();
        assert_eq!(remainder, [0u8; 0]);
        assert_eq!(parsed_packet, packet);
    }

    #[test]
    fn to_and_from_bytes_without_checksum() {
        let packet = Outbound::new(Command([0, 1]), vec![2]);
        let packet_bytes = packet.bytes_without_checksum();
        let (remainder, parsed_packet) =
            Outbound::take_without_checksum::<VerboseError<_>>(&packet_bytes).unwrap();
        assert_eq!(remainder, [0u8; 0]);
        assert_eq!(parsed_packet, packet);
    }
}

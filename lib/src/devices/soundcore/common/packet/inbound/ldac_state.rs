use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::common::{
    packet::{self, outbound::ToPacket},
    structures,
};

use super::FromPacketBody;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LdacState(pub structures::Ldac);

impl LdacState {
    pub const COMMAND: packet::Command = packet::Command([0x01, 0x7F]);
}

impl FromPacketBody for LdacState {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "LdacState",
            all_consuming(map(structures::Ldac::take, LdacState)),
        )
        .parse_complete(input)
    }
}

impl ToPacket for LdacState {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> packet::Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.0.bytes().to_vec()
    }
}

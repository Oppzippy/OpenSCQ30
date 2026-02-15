use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3957,
    common::packet::{self, Command, inbound::FromPacketBody},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3957SoundModes {
    pub sound_modes: a3957::structures::SoundModes,
}

impl A3957SoundModes {
    pub const COMMAND: Command = Command([0x06, 0x01]);
}

impl FromPacketBody for A3957SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "A3957SoundModesUpdatePacket",
            all_consuming(map(a3957::structures::SoundModes::take, |sound_modes| {
                Self { sound_modes }
            })),
        )
        .parse_complete(input)
    }
}

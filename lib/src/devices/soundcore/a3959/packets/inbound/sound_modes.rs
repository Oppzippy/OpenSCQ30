use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3959,
    common::packet::{self, Command, inbound::FromPacketBody},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3959SoundModes {
    pub sound_modes: a3959::structures::SoundModes,
}

impl A3959SoundModes {
    pub const COMMAND: Command = Command([0x06, 0x01]);
}

impl FromPacketBody for A3959SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "A3959SoundModesUpdatePacket",
            all_consuming(map(a3959::structures::SoundModes::take, |sound_modes| {
                Self { sound_modes }
            })),
        )
        .parse_complete(input)
    }
}

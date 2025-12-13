use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3035,
    common::packet::{self, inbound::FromPacketBody},
};

pub struct SoundModesUpdatePacket {
    pub sound_modes: a3035::structures::SoundModes,
}

impl SoundModesUpdatePacket {
    pub const COMMAND: packet::Command = packet::Command([0x06, 0x01]);
}

impl FromPacketBody for SoundModesUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "SoundModesUpdatePacket",
            all_consuming(map(a3035::structures::SoundModes::take, |sound_modes| {
                Self { sound_modes }
            })),
        )
        .parse_complete(input)
    }
}

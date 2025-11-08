use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3040,
    common::packet::{self, Command, inbound::FromPacketBody},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoundModesUpdate(pub a3040::structures::SoundModes);

impl SoundModesUpdate {
    pub const COMMAND: Command = Command([0x06, 0x01]);
}

impl FromPacketBody for SoundModesUpdate {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "sound mode update packet",
            all_consuming(map(a3040::structures::SoundModes::take, |sound_modes| {
                Self(sound_modes)
            })),
        )
        .parse_complete(input)
    }
}

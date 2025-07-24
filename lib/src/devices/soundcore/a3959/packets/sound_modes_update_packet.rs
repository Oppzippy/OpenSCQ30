use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3959::structures::A3959SoundModes,
    standard::packets::{Command, inbound::InboundPacket},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct A3959SoundModesUpdatePacket {
    pub sound_modes: A3959SoundModes,
}

impl A3959SoundModesUpdatePacket {
    pub const COMMAND: Command = Command([0x06, 0x01]);
}

impl InboundPacket for A3959SoundModesUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3959SoundModesUpdatePacket, E> {
        context(
            "A3959SoundModesUpdatePacket",
            all_consuming(map(A3959SoundModes::take, |sound_modes| {
                A3959SoundModesUpdatePacket { sound_modes }
            })),
        )
        .parse_complete(input)
    }
}

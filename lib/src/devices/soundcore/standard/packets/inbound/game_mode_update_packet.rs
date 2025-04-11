use nom::{
    IResult,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::standard::{packets::parsing::take_bool, structures::Command};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameModeUpdatePacket {
    pub is_enabled: bool,
}

impl GameModeUpdatePacket {
    pub const COMMAND: Command = Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x11]);
}

impl InboundPacket for GameModeUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], GameModeUpdatePacket, E> {
        context(
            "GameModeUpdatePacket",
            all_consuming(map(take_bool, |is_enabled| GameModeUpdatePacket {
                is_enabled,
            })),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::soundcore::standard::packets::inbound::{
        GameModeUpdatePacket, InboundPacket, take_inbound_packet_header,
    };

    #[test]
    fn it_parses_a_known_good_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x11, 0x0c, 0x00, 0x01, 0x28,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = GameModeUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        assert!(packet.is_enabled);
    }
}

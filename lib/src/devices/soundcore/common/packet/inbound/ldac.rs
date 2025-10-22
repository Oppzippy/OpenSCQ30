use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::common::packet::{self, Command, parsing::take_bool};

use super::FromPacketBody;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ldac {
    pub is_enabled: bool,
}

impl Ldac {
    #[allow(unused)]
    pub const COMMAND: Command = Command([0x01, 0x7F]);
}

impl FromPacketBody for Ldac {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "LdacStateUpdatePacket",
            all_consuming(map(take_bool, |is_enabled| Self { is_enabled })),
        )
        .parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::packet::{
        self,
        inbound::{FromPacketBody, Ldac},
    };

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x7F, 0x0b, 0x00, 0x01, 0x95,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let packet = Ldac::take::<VerboseError<_>>(&packet.body).unwrap().1;
        assert!(packet.is_enabled);
    }
}

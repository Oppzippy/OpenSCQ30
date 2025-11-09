use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::common::{
    packet::{self, Command},
    structures::GamingMode,
};

use super::FromPacketBody;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GamingModeUpdate(pub GamingMode);

impl GamingModeUpdate {
    #[allow(unused)]
    pub const COMMAND: Command = Command([0x01, 0x11]);
}

impl FromPacketBody for GamingModeUpdate {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "gaming mode update packet",
            all_consuming(map(GamingMode::take, Self)),
        )
        .parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::packet::{
        self,
        inbound::{FromPacketBody, GamingModeUpdate},
    };

    #[test]
    fn it_parses_a_known_good_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x11, 0x0b, 0x00, 0x01, 0x27,
        ];
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(input).unwrap();
        let packet = GamingModeUpdate::take::<VerboseError<_>>(&packet.body)
            .unwrap()
            .1;
        assert!(packet.0.is_enabled);
    }
}

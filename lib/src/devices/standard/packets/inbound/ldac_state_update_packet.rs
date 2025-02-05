use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    IResult,
};

use crate::devices::standard::{packets::parsing::take_bool, structures::Command};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LdacStateUpdatePacket {
    pub is_enabled: bool,
}

impl InboundPacket for LdacStateUpdatePacket {
    fn command() -> Command {
        Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x7F])
    }

    #[allow(dead_code)]
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], LdacStateUpdatePacket, E> {
        context(
            "LdacStateUpdatePacket",
            all_consuming(map(take_bool, |is_enabled| LdacStateUpdatePacket {
                is_enabled,
            })),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::packets::inbound::{
        take_inbound_packet_header, InboundPacket, LdacStateUpdatePacket,
    };

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x7F, 0x0c, 0x00, 0x01, 0x96,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = LdacStateUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        assert!(packet.is_enabled);
    }
}

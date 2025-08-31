use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::common::{packet::Command, structures};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TwsStatus(pub structures::TwsStatus);

impl TwsStatus {
    pub const COMMAND: Command = Command([0x01, 0x02]);
}

impl InboundPacket for TwsStatus {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "TwsStatusUpdatePacket",
            all_consuming(map(structures::TwsStatus::take, TwsStatus)),
        )
        .parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::{
        packet::{
            Packet,
            inbound::{InboundPacket, TwsStatus},
        },
        structures::HostDevice,
    };

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x02, 0x0c, 0x00, 0x00, 0x01, 0x19,
        ];
        let (_, packet) = Packet::take::<VerboseError<_>>(input).unwrap();
        let packet = TwsStatus::take::<VerboseError<_>>(&packet.body).unwrap().1;
        assert_eq!(HostDevice::Left, packet.0.host_device);
        assert!(packet.0.is_connected);
    }
}

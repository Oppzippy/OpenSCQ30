use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::standard::structures::{Command, TwsStatus};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TwsStatusUpdatePacket(pub TwsStatus);

impl TwsStatusUpdatePacket {
    pub const COMMAND: Command = Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x02]);
}

impl InboundPacket for TwsStatusUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], TwsStatusUpdatePacket, E> {
        context(
            "TwsStatusUpdatePacket",
            all_consuming(map(TwsStatus::take, TwsStatusUpdatePacket)),
        )
        .parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::standard::{
        packets::inbound::{InboundPacket, TwsStatusUpdatePacket, take_inbound_packet_header},
        structures::HostDevice,
    };

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x02, 0x0c, 0x00, 0x00, 0x01, 0x19,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = TwsStatusUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        assert_eq!(HostDevice::Left, packet.0.host_device);
        assert!(packet.0.is_connected);
    }
}

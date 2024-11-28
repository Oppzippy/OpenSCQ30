use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    sequence::tuple,
    IResult,
};

use crate::devices::standard::{
    packets::parsing::take_bool,
    structures::{Command, HostDevice},
};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TwsStatusUpdatePacket {
    pub host_device: HostDevice,
    pub tws_status: bool,
}

impl InboundPacket for TwsStatusUpdatePacket {
    fn command() -> Command {
        Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x02])
    }

    #[allow(dead_code)]
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], TwsStatusUpdatePacket, E> {
        context(
            "TwsStatusUpdatePacket",
            all_consuming(map(
                tuple((HostDevice::take, take_bool)),
                |(host_device, tws_status)| TwsStatusUpdatePacket {
                    host_device,
                    tws_status,
                },
            )),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::{
        packets::inbound::{take_inbound_packet_header, InboundPacket, TwsStatusUpdatePacket},
        structures::HostDevice,
    };

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x02, 0x0c, 0x00, 0x02, 0x01, 0x1b,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = TwsStatusUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        assert_eq!(HostDevice::Right, packet.host_device);
        assert_eq!(true, packet.tws_status);
    }
}

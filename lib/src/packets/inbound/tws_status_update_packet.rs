use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};

use crate::packets::parsing::{take_bool, ParseResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TwsStatusUpdatePacket {
    pub host_device: u8,
    pub tws_status: bool,
}

pub fn take_tws_status_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<TwsStatusUpdatePacket, E> {
    context(
        "TwsStatusUpdatePacket",
        all_consuming(map(
            tuple((le_u8, take_bool)),
            |(host_device, tws_status)| TwsStatusUpdatePacket {
                host_device,
                tws_status,
            },
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::packets::inbound::InboundPacket;

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x02, 0x0c, 0x00, 0x02, 0x01, 0x1b,
        ];
        let InboundPacket::TwsStatusUpdate(packet) = InboundPacket::new(input).unwrap() else {
            panic!("wrong packet type");
        };
        assert_eq!(2, packet.host_device);
        assert_eq!(true, packet.tws_status);
    }
}

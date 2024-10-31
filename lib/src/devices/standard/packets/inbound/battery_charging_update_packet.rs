use nom::{
    combinator::{all_consuming, map, opt},
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::devices::standard::{
    packets::parsing::ParseResult,
    structures::{Command, IsBatteryCharging},
};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatteryChargingUpdatePacket {
    pub left: IsBatteryCharging,
    pub right: Option<IsBatteryCharging>,
}

impl InboundPacket for BatteryChargingUpdatePacket {
    fn header() -> Command {
        Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x04])
    }

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<BatteryChargingUpdatePacket, E> {
        context(
            "BatteryChargingUpdatePacket",
            all_consuming(map(
                tuple((IsBatteryCharging::take, opt(IsBatteryCharging::take))),
                |(left, right)| BatteryChargingUpdatePacket { left, right },
            )),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::{
        packets::inbound::{
            take_inbound_packet_header, BatteryChargingUpdatePacket, InboundPacket,
        },
        structures::IsBatteryCharging,
    };

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x04, 0x0c, 0x00, 0x01, 0x00, 0x1b,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = BatteryChargingUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;

        assert_eq!(IsBatteryCharging::Yes, packet.left);
        assert_eq!(Some(IsBatteryCharging::No), packet.right);
    }

    #[test]
    fn it_parses_an_actual_packet_from_q30() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x04, 0x0b, 0x00, 0x01, 0x1a,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = BatteryChargingUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;

        assert_eq!(IsBatteryCharging::Yes, packet.left);
        assert_eq!(None, packet.right);
    }
}

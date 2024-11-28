use nom::{
    combinator::{all_consuming, map, opt},
    error::{context, ContextError, ParseError},
    sequence::tuple,
    IResult,
};

use crate::devices::standard::structures::{BatteryLevel, Command};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatteryLevelUpdatePacket {
    pub left: BatteryLevel,
    pub right: Option<BatteryLevel>,
}

impl InboundPacket for BatteryLevelUpdatePacket {
    fn command() -> Command {
        Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x03])
    }

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], BatteryLevelUpdatePacket, E> {
        context(
            "BatteryLevelUpdatePacket",
            all_consuming(map(
                tuple((
                    BatteryLevel::take,
                    opt(BatteryLevel::take),
                    opt(BatteryLevel::take),
                    opt(BatteryLevel::take),
                )),
                // TODO unsure what new_left and new_right are
                |(left, right, _new_left, _new_right)| BatteryLevelUpdatePacket { left, right },
            )),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::{
        packets::inbound::{take_inbound_packet_header, BatteryLevelUpdatePacket, InboundPacket},
        structures::BatteryLevel,
    };

    #[test]
    fn it_parses_a_manually_crafted_packet_without_new_battery() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x03, 0x0c, 0x00, 0x03, 0x04, 0x20,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = BatteryLevelUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        assert_eq!(BatteryLevel(3), packet.left);
        assert_eq!(Some(BatteryLevel(4)), packet.right);
    }

    #[test]
    fn it_parses_a_manually_crafted_packet_with_new_battery() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x03, 0x0e, 0x00, 0x04, 0x05, 0x01, 0x02, 0x27,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = BatteryLevelUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        assert_eq!(BatteryLevel(4), packet.left);
        assert_eq!(Some(BatteryLevel(5)), packet.right);
    }

    #[test]
    fn it_parses_an_actual_packet_from_q30() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x03, 0x0b, 0x00, 0x02, 0x1a,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = BatteryLevelUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        assert_eq!(BatteryLevel(2), packet.left);
        assert_eq!(None, packet.right);
    }
}

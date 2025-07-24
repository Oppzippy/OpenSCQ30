use nom::{
    IResult, Parser,
    combinator::{all_consuming, map, opt},
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::standard::{packet::Command, structures::BatteryLevel};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SingleBatteryLevelUpdatePacket {
    pub level: BatteryLevel,
}

impl SingleBatteryLevelUpdatePacket {
    pub const COMMAND: Command = Command([0x01, 0x03]);
}

impl InboundPacket for SingleBatteryLevelUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], SingleBatteryLevelUpdatePacket, E> {
        context(
            "SingleBatteryLevelUpdatePacket",
            all_consuming(map(BatteryLevel::take, |level| {
                SingleBatteryLevelUpdatePacket { level }
            })),
        )
        .parse_complete(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DualBatteryLevelUpdatePacket {
    pub left: BatteryLevel,
    pub right: BatteryLevel,
}

impl DualBatteryLevelUpdatePacket {
    pub const COMMAND: Command = Command([0x01, 0x03]);
}

impl InboundPacket for DualBatteryLevelUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], DualBatteryLevelUpdatePacket, E> {
        context(
            "DualBatteryLevelUpdatePacket",
            all_consuming(map(
                (
                    BatteryLevel::take,
                    BatteryLevel::take,
                    opt(BatteryLevel::take),
                    opt(BatteryLevel::take),
                ),
                // TODO unsure what new_left and new_right are
                |(left, right, _new_left, _new_right)| DualBatteryLevelUpdatePacket { left, right },
            )),
        )
        .parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::standard::packet::Packet;

    use super::*;

    #[test]
    fn it_parses_a_manually_crafted_packet_without_new_battery() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x03, 0x0c, 0x00, 0x03, 0x04, 0x20,
        ];
        let (_, packet) = Packet::take::<VerboseError<_>>(input).unwrap();
        let packet = DualBatteryLevelUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .unwrap()
            .1;
        assert_eq!(BatteryLevel(3), packet.left);
        assert_eq!(BatteryLevel(4), packet.right);
    }

    #[test]
    fn it_parses_a_manually_crafted_packet_with_new_battery() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x03, 0x0e, 0x00, 0x04, 0x05, 0x01, 0x02, 0x27,
        ];
        let (_, packet) = Packet::take::<VerboseError<_>>(input).unwrap();
        let packet = DualBatteryLevelUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .unwrap()
            .1;
        assert_eq!(BatteryLevel(4), packet.left);
        assert_eq!(BatteryLevel(5), packet.right);
    }

    #[test]
    fn it_parses_an_actual_packet_from_q30() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x03, 0x0b, 0x00, 0x02, 0x1a,
        ];
        let (_, packet) = Packet::take::<VerboseError<_>>(input).unwrap();
        let packet = SingleBatteryLevelUpdatePacket::take::<VerboseError<_>>(&packet.body)
            .unwrap()
            .1;
        assert_eq!(BatteryLevel(2), packet.level);
    }
}

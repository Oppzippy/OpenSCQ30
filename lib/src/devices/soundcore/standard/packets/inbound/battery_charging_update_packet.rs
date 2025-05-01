use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
    sequence::pair,
};

use crate::devices::soundcore::standard::structures::{Command, IsBatteryCharging};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SingleBatteryChargingUpdatePacket {
    pub is_charging: IsBatteryCharging,
}

impl SingleBatteryChargingUpdatePacket {
    pub const COMMAND: Command = Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x04]);
}

impl InboundPacket for SingleBatteryChargingUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], SingleBatteryChargingUpdatePacket, E> {
        context(
            "SingleBatteryChargingUpdatePacket",
            all_consuming(map(IsBatteryCharging::take, |is_charging| {
                SingleBatteryChargingUpdatePacket { is_charging }
            })),
        )
        .parse_complete(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DualBatteryChargingUpdatePacket {
    pub left: IsBatteryCharging,
    pub right: IsBatteryCharging,
}

impl DualBatteryChargingUpdatePacket {
    pub const COMMAND: Command = Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x04]);
}

impl InboundPacket for DualBatteryChargingUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], DualBatteryChargingUpdatePacket, E> {
        context(
            "DualBatteryChargingUpdatePacket",
            all_consuming(map(
                pair(IsBatteryCharging::take, IsBatteryCharging::take),
                |(left, right)| DualBatteryChargingUpdatePacket { left, right },
            )),
        )
        .parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::standard::packets::inbound::take_inbound_packet_header;

    use super::*;

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x04, 0x0c, 0x00, 0x01, 0x00, 0x1b,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = DualBatteryChargingUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;

        assert_eq!(IsBatteryCharging::Yes, packet.left);
        assert_eq!(IsBatteryCharging::No, packet.right);
    }

    #[test]
    fn it_parses_an_actual_packet_from_q30() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x04, 0x0b, 0x00, 0x01, 0x1a,
        ];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        let packet = SingleBatteryChargingUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;

        assert_eq!(IsBatteryCharging::Yes, packet.is_charging);
    }
}

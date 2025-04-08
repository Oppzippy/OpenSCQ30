use nom::{
    IResult,
    combinator::{all_consuming, map, opt},
    error::{ContextError, ParseError, context},
    sequence::tuple,
};

use crate::devices::soundcore::standard::structures::{Command, IsBatteryCharging};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatteryChargingUpdatePacket {
    pub left: IsBatteryCharging,
    pub right: Option<IsBatteryCharging>,
}

impl BatteryChargingUpdatePacket {
    pub const COMMAND: Command = Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x04]);
}

impl InboundPacket for BatteryChargingUpdatePacket {
    fn command() -> Command {
        Self::COMMAND
    }

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], BatteryChargingUpdatePacket, E> {
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

    use crate::devices::soundcore::standard::{
        packets::inbound::{
            BatteryChargingUpdatePacket, InboundPacket, take_inbound_packet_header,
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

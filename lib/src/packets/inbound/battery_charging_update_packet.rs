use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::packets::{
    parsing::{take_is_battery_charging, ParseResult},
    structures::IsBatteryCharging,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatteryChargingUpdatePacket {
    pub left: IsBatteryCharging,
    pub right: IsBatteryCharging,
}

pub fn take_battery_charging_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<BatteryChargingUpdatePacket, E> {
    context(
        "BatteryChargingUpdatePacket",
        all_consuming(map(
            tuple((take_is_battery_charging, take_is_battery_charging)),
            |(left, right)| BatteryChargingUpdatePacket { left, right },
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::packets::{
        inbound::take_battery_charging_update_packet,
        parsing::{take_checksum, take_packet_header},
        structures::IsBatteryCharging,
    };

    fn strip(input: &[u8]) -> &[u8] {
        let input = take_checksum::<VerboseError<&[u8]>>(input).unwrap().0;
        let input = take_packet_header::<VerboseError<&[u8]>>(input).unwrap().0;
        input
    }

    #[test]
    fn it_parses_a_manually_crafted_packet() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x04, 0x0c, 0x00, 0x01, 0x00, 0x1b,
        ];
        let input = strip(input);
        let packet = take_battery_charging_update_packet::<VerboseError<&[u8]>>(input)
            .unwrap()
            .1;
        assert_eq!(IsBatteryCharging::Yes, packet.left);
        assert_eq!(IsBatteryCharging::No, packet.right);
    }
}

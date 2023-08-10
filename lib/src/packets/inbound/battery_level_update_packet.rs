use nom::{
    combinator::{all_consuming, map, opt},
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::packets::{
    parsing::{take_battery_level, ParseResult},
    structures::BatteryLevel,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatteryLevelUpdatePacket {
    pub left: BatteryLevel,
    pub right: BatteryLevel,
}

pub fn take_battery_level_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<BatteryLevelUpdatePacket, E> {
    context(
        "BatteryLevelUpdatePacket",
        all_consuming(map(
            tuple((
                take_battery_level,
                take_battery_level,
                opt(take_battery_level),
                opt(take_battery_level),
            )),
            // TODO unsure what new_left and new_right are
            |(left, right, _new_left, _new_right)| BatteryLevelUpdatePacket { left, right },
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::packets::{
        inbound::take_battery_level_update_packet,
        parsing::{take_checksum, take_packet_header},
        structures::BatteryLevel,
    };

    fn strip(input: &[u8]) -> &[u8] {
        let input = take_checksum::<VerboseError<&[u8]>>(input).unwrap().0;
        let input = take_packet_header::<VerboseError<&[u8]>>(input).unwrap().0;
        input
    }

    #[test]
    fn it_parses_a_manually_crafted_packet_without_new_battery() {
        let input: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x03, 0x0c, 0x00, 0x03, 0x04, 0x20,
        ];
        let input = strip(input);
        let packet = take_battery_level_update_packet::<VerboseError<&[u8]>>(input)
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
        let input = strip(input);
        let packet = take_battery_level_update_packet::<VerboseError<&[u8]>>(input)
            .unwrap()
            .1;
        assert_eq!(BatteryLevel(4), packet.left);
        assert_eq!(BatteryLevel(5), packet.right);
    }
}

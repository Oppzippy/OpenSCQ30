use nom::{
    bytes::complete::take,
    combinator::map_opt,
    error::{context, ContextError, ParseError},
};

use crate::packets::structures::PacketType;

use super::ParseResult;

pub fn take_packet_type<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<PacketType, E> {
    context(
        "packet type 7 byte prefix",
        map_opt(take(7usize), |prefix: &[u8]| match prefix {
            [0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01] => Some(PacketType::SoundModeUpdate),
            [0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81] => Some(PacketType::SetSoundModeOk),
            [0x09, 0xff, 0x00, 0x00, 0x01, 0x02, 0x81] => Some(PacketType::SetEqualizerOk),
            [0x09, 0xff, 0x00, 0x00, 0x01, 0x02, 0x83] => Some(PacketType::SetEqualizerWithDrcOk),
            [0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01] => Some(PacketType::StateUpdate),
            [0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x02] => Some(PacketType::TwsStatusUpdate),
            [0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x03] => Some(PacketType::BatteryLevelUpdate),
            [0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x04] => Some(PacketType::BatteryChargingUpdate),
            [0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x05] => Some(PacketType::FirmwareVersionUpdate),
            [0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x7F] => Some(PacketType::LdacStateUpdate),
            _ => None,
        }),
    )(input)
}

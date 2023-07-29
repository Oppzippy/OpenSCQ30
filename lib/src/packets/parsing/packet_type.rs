use nom::{
    bytes::complete::take,
    combinator::map_opt,
    error::{context, ContextError, ParseError},
};

use crate::packets::structures::PacketType;

use super::ParseResult;

const SOUND_MODE_UPDATE_PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01];
const SET_SOUND_MODE_OK_PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81];
const SET_EQUALIZER_OK_PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x02, 0x81];
const STATE_UPDATE_PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01];

pub fn take_packet_type<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<PacketType, E> {
    context(
        "packet type 7 byte prefix",
        map_opt(take(7usize), |prefix| match prefix {
            SOUND_MODE_UPDATE_PREFIX => Some(PacketType::SoundModeUpdate),
            SET_SOUND_MODE_OK_PREFIX => Some(PacketType::SetSoundModeOk),
            SET_EQUALIZER_OK_PREFIX => Some(PacketType::SetEqualizerOk),
            STATE_UPDATE_PREFIX => Some(PacketType::StateUpdate),
            _ => None,
        }),
    )(input)
}

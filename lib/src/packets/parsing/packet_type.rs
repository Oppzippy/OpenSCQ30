use nom::{
    bytes::complete::take,
    combinator::map_res,
    error::{context, ErrorKind},
};

use crate::packets::structures::PacketType;

use super::ParseResult;

const SOUND_MODE_UPDATE_PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01];
const SET_SOUND_MODE_OK_PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81];
const SET_EQUALIZER_OK_PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x02, 0x81];
const STATE_UPDATE_PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01];

pub fn take_packet_type(input: &[u8]) -> ParseResult<PacketType> {
    context(
        "packet type 7 byte prefix",
        map_res(take(7usize), |prefix| match prefix {
            SOUND_MODE_UPDATE_PREFIX => Ok(PacketType::SoundModeUpdate),
            SET_SOUND_MODE_OK_PREFIX => Ok(PacketType::SetSoundModeOk),
            SET_EQUALIZER_OK_PREFIX => Ok(PacketType::SetEqualizerOk),
            STATE_UPDATE_PREFIX => Ok(PacketType::StateUpdate),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                input,
                ErrorKind::OneOf,
            ))),
        }),
    )(input)
}

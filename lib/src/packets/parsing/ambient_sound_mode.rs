use nom::{
    combinator::map_res,
    error::{context, ErrorKind},
    number::complete::le_u8,
};

use crate::packets::structures::AmbientSoundMode;

use super::ParseResult;

pub fn take_ambient_sound_mode(input: &[u8]) -> ParseResult<AmbientSoundMode> {
    context(
        "ambient sound mode",
        map_res(le_u8, |ambient_sound_mode_id| {
            AmbientSoundMode::from_id(ambient_sound_mode_id)
                .ok_or_else(|| nom::Err::Failure(nom::error::Error::new(input, ErrorKind::Alt)))
        }),
    )(input)
}

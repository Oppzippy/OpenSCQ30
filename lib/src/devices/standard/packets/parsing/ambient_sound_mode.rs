use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::devices::standard::structures::AmbientSoundMode;

use super::ParseResult;

pub fn take_ambient_sound_mode<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<AmbientSoundMode, E> {
    context(
        "ambient sound mode",
        map(le_u8, |ambient_sound_mode_id| {
            AmbientSoundMode::from_id(ambient_sound_mode_id).unwrap_or_default()
        }),
    )(input)
}

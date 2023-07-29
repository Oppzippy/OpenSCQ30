use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::packets::structures::SoundModes;

use super::{
    take_ambient_sound_mode, take_custom_noise_canceling, take_noise_canceling_mode,
    take_transparency_mode, ParseResult,
};

pub fn take_sound_modes<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<SoundModes, E> {
    context(
        "group of sound modes",
        map(
            tuple((
                take_ambient_sound_mode,
                take_noise_canceling_mode,
                take_transparency_mode,
                take_custom_noise_canceling,
            )),
            |(
                ambient_sound_mode,
                noise_canceling_mode,
                transparency_mode,
                custom_noise_canceling,
            )| {
                SoundModes {
                    ambient_sound_mode,
                    noise_canceling_mode,
                    transparency_mode,
                    custom_noise_canceling,
                }
            },
        ),
    )(input)
}

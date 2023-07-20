use nom::{combinator::map, error::context, sequence::tuple};

use crate::packets::structures::SoundModes;

use super::{
    take_ambient_sound_mode, take_custom_noise_canceling, take_noise_canceling_mode,
    take_transparency_mode, ParseResult,
};

pub fn take_sound_modes(input: &[u8]) -> ParseResult<SoundModes> {
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

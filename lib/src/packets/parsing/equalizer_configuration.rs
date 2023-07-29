use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u16,
    sequence::pair,
};

use crate::packets::structures::{EqualizerConfiguration, PresetEqualizerProfile};

use super::{take_volume_adjustments, ParseResult};

pub fn take_equalizer_configuration<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<EqualizerConfiguration, E> {
    context(
        "equalizer configuration",
        map(
            pair(le_u16, take_volume_adjustments),
            |(profile_id, volume_adjustments)| {
                PresetEqualizerProfile::from_id(profile_id)
                    .map(EqualizerConfiguration::new_from_preset_profile)
                    .unwrap_or(EqualizerConfiguration::new_custom_profile(
                        volume_adjustments,
                    ))
            },
        ),
    )(input)
}

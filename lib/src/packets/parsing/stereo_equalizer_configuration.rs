use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::packets::structures::EqualizerConfiguration;

use super::{take_equalizer_configuration, take_volume_adjustments, ParseResult};

pub fn take_stereo_equalizer_configuration<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<EqualizerConfiguration, E> {
    context(
        "stereo equalizer configuration",
        map(
            tuple((take_equalizer_configuration, take_volume_adjustments)),
            // discard the right channel since we don't support it yet
            |results| results.0,
        ),
    )(input)
}

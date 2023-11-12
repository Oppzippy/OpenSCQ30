use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::devices::standard::structures::EqualizerConfiguration;

use super::{take_equalizer_configuration, take_volume_adjustments, ParseResult};

pub fn take_stereo_equalizer_configuration<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    num_bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<EqualizerConfiguration, E> {
    move |input| {
        context(
            "stereo equalizer configuration",
            map(
                tuple((
                    take_equalizer_configuration(num_bands),
                    take_volume_adjustments(num_bands),
                )),
                // discard the right channel since we don't support it yet
                |results| results.0,
            ),
        )(input)
    }
}

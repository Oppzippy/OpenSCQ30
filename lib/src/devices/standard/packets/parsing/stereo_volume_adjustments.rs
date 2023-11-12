use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::devices::standard::structures::StereoVolumeAdjustments;

use super::{take_volume_adjustments, ParseResult};

pub fn take_stereo_volume_adjustments<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    num_bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<StereoVolumeAdjustments, E> {
    move |input| {
        context(
            "stereo volume adjustments",
            map(
                tuple((
                    take_volume_adjustments(num_bands),
                    take_volume_adjustments(num_bands),
                )),
                |(left, right)| StereoVolumeAdjustments { left, right },
            ),
        )(input)
    }
}

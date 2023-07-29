use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::packets::structures::StereoVolumeAdjustments;

use super::{take_volume_adjustments, ParseResult};

pub fn take_stereo_volume_adjustments<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<StereoVolumeAdjustments, E> {
    context(
        "stereo volume adjustments",
        map(
            tuple((take_volume_adjustments, take_volume_adjustments)),
            |(left, right)| StereoVolumeAdjustments { left, right },
        ),
    )(input)
}

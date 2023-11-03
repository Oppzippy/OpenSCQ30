use nom::{
    bytes::complete::take,
    combinator::map,
    error::{context, ContextError, ParseError},
};

use crate::packets::structures::VolumeAdjustments;

use super::ParseResult;

pub fn take_volume_adjustments<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<VolumeAdjustments, E> {
    context(
        "volume adjustment",
        map(take(8usize), |volume_adjustment_bytes: &[u8]| {
            // we already verified the length, so we can unwrap
            VolumeAdjustments::from_bytes(volume_adjustment_bytes)
        }),
    )(input)
}

use nom::{
    bytes::complete::take,
    combinator::map,
    error::{context, ContextError, ParseError},
};

use crate::packets::structures::VolumeAdjustments;

use super::ParseResult;

pub fn take_volume_adjustments<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    num_bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<VolumeAdjustments, E> {
    move |input| {
        context(
            "volume adjustment",
            map(take(num_bands), |volume_adjustment_bytes: &[u8]| {
                VolumeAdjustments::from_bytes(volume_adjustment_bytes)
                    .expect("length was already verified by take(8)")
            }),
        )(input)
    }
}

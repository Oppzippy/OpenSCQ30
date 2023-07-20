use nom::{bytes::complete::take, combinator::map, error::context};

use crate::packets::structures::VolumeAdjustments;

use super::ParseResult;

pub fn take_volume_adjustments(input: &[u8]) -> ParseResult<VolumeAdjustments> {
    context(
        "volume adjustment",
        map(take(8usize), |volume_adjustment_bytes: &[u8]| {
            // we already verified the length, so we can unwrap
            VolumeAdjustments::from_bytes(volume_adjustment_bytes.try_into().unwrap())
        }),
    )(input)
}

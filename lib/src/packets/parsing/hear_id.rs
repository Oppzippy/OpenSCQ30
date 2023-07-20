use nom::{combinator::map, error::context, number::complete::le_i32, sequence::tuple};

use crate::packets::structures::HearId;

use super::{take_bool, take_volume_adjustments, ParseResult};

pub fn take_hear_id(input: &[u8]) -> ParseResult<HearId> {
    context(
        "hear id",
        map(
            tuple((
                take_bool,
                take_volume_adjustments,
                take_volume_adjustments,
                le_i32,
            )),
            |(is_enabled, left, right, time)| HearId {
                is_enabled,
                left,
                right,
                time,
            },
        ),
    )(input)
}

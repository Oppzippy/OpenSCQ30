use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_i32,
    sequence::tuple,
};

use crate::devices::standard::structures::BasicHearId;

use super::{take_bool, take_stereo_volume_adjustments, ParseResult};

pub fn take_basic_hear_id<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<BasicHearId, E> {
    context(
        "hear id",
        map(
            tuple((take_bool, take_stereo_volume_adjustments(8), le_i32)),
            |(is_enabled, volume_adjustments, time)| BasicHearId {
                is_enabled,
                volume_adjustments,
                time,
            },
        ),
    )(input)
}

use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_i32,
    sequence::tuple,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::devices::standard::packets::parsing::take_bool;

use super::StereoVolumeAdjustments;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BasicHearId {
    pub is_enabled: bool,
    pub volume_adjustments: StereoVolumeAdjustments,
    pub time: i32,
}

impl BasicHearId {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], BasicHearId, E> {
        context(
            "basic hear id",
            map(
                tuple((take_bool, StereoVolumeAdjustments::take(8), le_i32)),
                |(is_enabled, volume_adjustments, time)| BasicHearId {
                    is_enabled,
                    volume_adjustments,
                    time,
                },
            ),
        )(input)
    }
}

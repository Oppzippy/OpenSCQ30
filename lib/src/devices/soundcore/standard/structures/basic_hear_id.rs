use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::le_i32,
    sequence::tuple,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::devices::soundcore::standard::packets::parsing::take_bool;

use super::VolumeAdjustments;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BasicHearId {
    pub is_enabled: bool,
    pub volume_adjustments: Vec<VolumeAdjustments>,
    pub time: i32,
}

impl Default for BasicHearId {
    fn default() -> Self {
        Self {
            is_enabled: Default::default(),
            volume_adjustments: vec![Default::default(), Default::default()],
            time: Default::default(),
        }
    }
}

impl BasicHearId {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], BasicHearId, E> {
        context(
            "basic hear id",
            map(
                tuple((take_bool, count(VolumeAdjustments::take(8), 2), le_i32)),
                |(is_enabled, volume_adjustments, time)| BasicHearId {
                    is_enabled,
                    volume_adjustments,
                    time,
                },
            ),
        )(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [self.is_enabled as u8]
            .into_iter()
            .chain(self.volume_adjustments.iter().map(|v| v.bytes()).flatten())
            .chain(self.time.to_le_bytes())
    }
}

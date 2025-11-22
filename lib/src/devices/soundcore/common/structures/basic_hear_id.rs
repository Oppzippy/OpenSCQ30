use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::le_i32,
};

use crate::devices::soundcore::common::packet::parsing::take_bool;

use super::CommonVolumeAdjustments;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicHearId<const C: usize, const B: usize> {
    pub is_enabled: bool,
    pub volume_adjustments: [CommonVolumeAdjustments<B>; C],
    pub time: i32,
}

impl<const C: usize, const B: usize> Default for BasicHearId<C, B> {
    fn default() -> Self {
        Self {
            is_enabled: Default::default(),
            volume_adjustments: [Default::default(); C],
            time: Default::default(),
        }
    }
}

impl<const C: usize, const B: usize> BasicHearId<C, B> {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "basic hear id",
            map(
                (take_bool, count(CommonVolumeAdjustments::take, 2), le_i32),
                |(is_enabled, volume_adjustments, time)| Self {
                    is_enabled,
                    volume_adjustments: volume_adjustments
                        .try_into()
                        .expect("count is guaranteed to return a vec with the specified length"),
                    time,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        [self.is_enabled as u8]
            .into_iter()
            .chain(self.volume_adjustments.iter().flat_map(|v| v.bytes()))
            .chain(self.time.to_le_bytes())
    }
}

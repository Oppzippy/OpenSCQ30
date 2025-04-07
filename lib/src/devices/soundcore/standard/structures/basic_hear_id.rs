use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::le_i32,
    sequence::tuple,
};

use crate::devices::soundcore::standard::packets::parsing::take_bool;

use super::VolumeAdjustments;

#[derive(Debug, Clone, PartialEq)]
pub struct BasicHearId<const C: usize, const B: usize> {
    pub is_enabled: bool,
    pub volume_adjustments: [VolumeAdjustments<B>; C],
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
                tuple((take_bool, count(VolumeAdjustments::take, 2), le_i32)),
                |(is_enabled, volume_adjustments, time)| BasicHearId {
                    is_enabled,
                    volume_adjustments: volume_adjustments
                        .try_into()
                        .expect("count is guaranteed to return a vec with the specified length"),
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

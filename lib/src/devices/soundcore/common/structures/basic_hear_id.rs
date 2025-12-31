use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::be_u32,
};

use crate::devices::soundcore::common::packet::parsing::take_bool;

use super::CommonVolumeAdjustments;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicHearId<const C: usize, const B: usize> {
    pub is_enabled: bool,
    pub volume_adjustments: [Option<CommonVolumeAdjustments<B>>; C],
    pub time: u32,
}

impl<const C: usize, const B: usize> Default for BasicHearId<C, B> {
    fn default() -> Self {
        Self {
            is_enabled: Default::default(),
            volume_adjustments: [None; C],
            time: Default::default(),
        }
    }
}

impl<const CHANNELS: usize, const BANDS: usize> BasicHearId<CHANNELS, BANDS> {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "basic hear id",
            map(
                (
                    take_bool,
                    count(CommonVolumeAdjustments::take_optional, 2),
                    be_u32,
                ),
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
            .chain(self.volume_adjustment_bytes())
            .chain(self.time.to_be_bytes())
    }

    pub fn volume_adjustment_bytes(&self) -> impl Iterator<Item = u8> {
        self.volume_adjustments
            .iter()
            .flat_map(|maybe_volume_adjustments| {
                maybe_volume_adjustments.map_or([0xFF; BANDS], |v| v.bytes())
            })
    }
}

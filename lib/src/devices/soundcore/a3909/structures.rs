use std::iter;

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::be_i32,
};

use crate::devices::soundcore::common::{
    self, packet::parsing::take_bool, structures::VolumeAdjustments,
};

pub type EqualizerConfiguration = common::structures::EqualizerConfiguration<2, 8, -12, 12, 0>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HearId {
    pub is_enabled: bool,
    pub volume_adjustments: [Option<VolumeAdjustments<8, -12, 12, 0>>; 2],
    pub time: i32,
}

impl HearId {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "hear id",
            map(
                (
                    take_bool,
                    (
                        VolumeAdjustments::take_optional,
                        VolumeAdjustments::take_optional,
                    ),
                    be_i32,
                ),
                |(is_enabled, volume_adjustments, time)| Self {
                    is_enabled,
                    volume_adjustments: volume_adjustments.into(),
                    time,
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        iter::once(self.is_enabled as u8)
            .chain(
                self.volume_adjustments
                    .iter()
                    .flat_map(|maybe_adjustments| {
                        maybe_adjustments
                            .map(|adjustments| adjustments.bytes())
                            .unwrap_or([0xFF; 8])
                    }),
            )
            .chain(self.time.to_be_bytes())
    }
}

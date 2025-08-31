use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::le_i32,
};

use crate::devices::soundcore::common::packet::parsing::take_bool;

use super::{HearIdMusicType, HearIdType, VolumeAdjustments};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomHearId<const C: usize, const B: usize> {
    pub is_enabled: bool,
    pub volume_adjustments: [VolumeAdjustments<B>; C],
    pub time: i32,
    pub hear_id_type: HearIdType,
    pub hear_id_music_type: HearIdMusicType,
    pub custom_volume_adjustments: Option<[VolumeAdjustments<B>; C]>,
}

impl<const C: usize, const B: usize> Default for CustomHearId<C, B> {
    fn default() -> Self {
        Self {
            is_enabled: Default::default(),
            volume_adjustments: [Default::default(); C],
            time: Default::default(),
            hear_id_type: Default::default(),
            hear_id_music_type: Default::default(),
            custom_volume_adjustments: Default::default(),
        }
    }
}

impl<const C: usize, const B: usize> CustomHearId<C, B> {
    pub(crate) fn take_with_all_fields<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "custom hear id",
            map(
                (
                    take_bool,
                    count(VolumeAdjustments::take, C),
                    le_i32,
                    HearIdType::take,
                    HearIdMusicType::take,
                    count(VolumeAdjustments::take, C),
                ),
                |(
                    is_enabled,
                    volume_adjustments,
                    time,
                    hear_id_type,
                    music_type,
                    custom_volume_adjustments,
                )| {
                    let volume_adjustments: [VolumeAdjustments<B>; C] = volume_adjustments
                        .try_into()
                        .expect("count is guaranteed to return a vec with the desired length");
                    // The first byte of the custom volume adjustments determines whether or not they're present
                    let custom_volume_adjustments: Option<[VolumeAdjustments<B>; C]> =
                        if custom_volume_adjustments[0].bytes()[0] != 255 {
                            Some(custom_volume_adjustments.try_into().expect(
                                "count is guaranteed to return a vec with the desired length",
                            ))
                        } else {
                            None
                        };
                    Self {
                        is_enabled,
                        volume_adjustments,
                        time,
                        hear_id_type,
                        hear_id_music_type: music_type,
                        custom_volume_adjustments,
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    // TODO maybe use a different struct for this?
    pub(crate) fn take_without_music_type<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "custom hear id without music_type",
            map(
                (
                    take_bool,
                    count(VolumeAdjustments::take, C),
                    le_i32,
                    HearIdType::take,
                    count(VolumeAdjustments::take, C),
                    take(2usize), // hear id eq index?
                ),
                |(
                    is_enabled,
                    volume_adjustments,
                    time,
                    hear_id_type,
                    custom_volume_adjustments,
                    _,
                )| {
                    Self {
                        is_enabled,
                        volume_adjustments: volume_adjustments
                            .try_into()
                            .expect("count is guaranteed to return a vec with the desired length"),
                        time,
                        hear_id_type,
                        hear_id_music_type: HearIdMusicType(0),
                        custom_volume_adjustments: Some(
                            custom_volume_adjustments.try_into().expect(
                                "count is guaranteed to return a vec with the desired length",
                            ),
                        ),
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

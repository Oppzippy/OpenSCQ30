use nom::{
    IResult,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_i32,
    sequence::tuple,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::devices::standard::packets::parsing::take_bool;

use super::{HearIdMusicType, HearIdType, StereoVolumeAdjustments, VolumeAdjustments};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct CustomHearId {
    pub is_enabled: bool,
    pub volume_adjustments: StereoVolumeAdjustments,
    pub time: i32,
    pub hear_id_type: HearIdType,
    pub hear_id_music_type: HearIdMusicType,
    pub custom_volume_adjustments: Option<StereoVolumeAdjustments>,
}

impl CustomHearId {
    pub(crate) fn take_with_all_fields<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], CustomHearId, E> {
        context(
            "custom hear id",
            map(
                tuple((
                    take_bool,
                    StereoVolumeAdjustments::take(8),
                    le_i32,
                    HearIdType::take,
                    HearIdMusicType::take,
                    take(8usize),
                    VolumeAdjustments::take(8),
                )),
                |(
                    is_enabled,
                    volume_adjustments,
                    time,
                    hear_id_type,
                    music_type,
                    custom_left_values,
                    custom_right,
                )| {
                    // The first byte of the custom volume adjustments determines whether or not they're present
                    let custom_volume_adjustments = if custom_left_values[0] != 255 {
                        let custom_left = VolumeAdjustments::from_bytes(custom_left_values)
                            .expect("length was already verified by take(8)");
                        Some(StereoVolumeAdjustments {
                            left: custom_left,
                            right: custom_right,
                        })
                    } else {
                        None
                    };
                    CustomHearId {
                        is_enabled,
                        volume_adjustments,
                        time,
                        hear_id_type,
                        hear_id_music_type: music_type,
                        custom_volume_adjustments,
                    }
                },
            ),
        )(input)
    }

    // TODO maybe use a different struct for this?
    pub(crate) fn take_without_music_type<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        num_bands: usize,
    ) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], CustomHearId, E> {
        context(
            "custom hear id without music_type",
            map(
                tuple((
                    take_bool,
                    StereoVolumeAdjustments::take(num_bands),
                    le_i32,
                    HearIdType::take,
                    StereoVolumeAdjustments::take(num_bands),
                    take(2usize), // hear id eq index?
                )),
                |(
                    is_enabled,
                    volume_adjustments,
                    time,
                    hear_id_type,
                    custom_volume_adjustments,
                    _,
                )| {
                    CustomHearId {
                        is_enabled,
                        volume_adjustments,
                        time,
                        hear_id_type,
                        hear_id_music_type: HearIdMusicType(0),
                        custom_volume_adjustments: Some(custom_volume_adjustments),
                    }
                },
            ),
        )
    }
}

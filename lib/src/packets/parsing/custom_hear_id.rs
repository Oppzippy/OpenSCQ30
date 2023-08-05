use nom::{
    bytes::complete::take,
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_i32,
    sequence::tuple,
};

use crate::packets::structures::{CustomHearId, StereoVolumeAdjustments, VolumeAdjustments};

use super::{
    take_bool, take_hear_id_music_type, take_hear_id_type, take_stereo_volume_adjustments,
    take_volume_adjustments, ParseResult,
};

pub fn take_custom_hear_id<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<CustomHearId, E> {
    context(
        "custom hear id",
        map(
            tuple((
                take_bool,
                take_stereo_volume_adjustments,
                le_i32,
                take_hear_id_type,
                take_hear_id_music_type,
                take(8usize),
                take_volume_adjustments,
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
                    let custom_left =
                        VolumeAdjustments::from_bytes(custom_left_values.try_into().unwrap());
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
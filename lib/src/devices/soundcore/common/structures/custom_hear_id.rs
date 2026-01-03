use std::iter;

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::be_u32,
};

use crate::devices::soundcore::common::{packet::parsing::take_bool, structures::HearIdMusicGenre};

use super::{CommonVolumeAdjustments, HearIdType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomHearId<const C: usize, const B: usize> {
    pub is_enabled: bool,
    pub volume_adjustments: [Option<CommonVolumeAdjustments<B>>; C],
    pub time: u32,
    pub hear_id_type: HearIdType,
    pub favorite_music_genre: HearIdMusicGenre,
    pub custom_volume_adjustments: [Option<CommonVolumeAdjustments<B>>; C],
}

impl<const C: usize, const B: usize> Default for CustomHearId<C, B> {
    fn default() -> Self {
        Self {
            is_enabled: Default::default(),
            volume_adjustments: [None; C],
            time: Default::default(),
            hear_id_type: Default::default(),
            favorite_music_genre: Default::default(),
            custom_volume_adjustments: [None; C],
        }
    }
}

impl<const CHANNELS: usize, const BANDS: usize> CustomHearId<CHANNELS, BANDS> {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "custom hear id",
            map(
                (
                    take_bool,
                    count(CommonVolumeAdjustments::take_optional, CHANNELS),
                    be_u32,
                    HearIdType::take,
                    HearIdMusicGenre::take_one_byte,
                    count(CommonVolumeAdjustments::take_optional, CHANNELS),
                ),
                |(
                    is_enabled,
                    volume_adjustments,
                    time,
                    hear_id_type,
                    music_type,
                    custom_volume_adjustments,
                )| {
                    // The first of volume adjustments determines whether or not they're present
                    let volume_adjustments: [Option<CommonVolumeAdjustments<BANDS>>; CHANNELS] =
                        volume_adjustments
                            .try_into()
                            .expect("count is guaranteed to return a vec with the desired length");
                    let custom_volume_adjustments: [Option<CommonVolumeAdjustments<BANDS>>;
                        CHANNELS] = custom_volume_adjustments
                        .try_into()
                        .expect("count is guaranteed to return a vec with the desired length");
                    Self {
                        is_enabled,
                        volume_adjustments,
                        time,
                        hear_id_type,
                        favorite_music_genre: music_type,
                        custom_volume_adjustments,
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn take_with_music_genre_at_end<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "custom hear id without music_type",
            map(
                (
                    take_bool,
                    count(CommonVolumeAdjustments::take_optional, CHANNELS),
                    be_u32,
                    HearIdType::take,
                    count(CommonVolumeAdjustments::take_optional, CHANNELS),
                    HearIdMusicGenre::take_two_bytes, // hear id eq index?
                ),
                |(
                    is_enabled,
                    volume_adjustments,
                    time,
                    hear_id_type,
                    custom_volume_adjustments,
                    favorite_music_genre,
                )| {
                    Self {
                        is_enabled,
                        volume_adjustments: volume_adjustments
                            .try_into()
                            .expect("count is guaranteed to return a vec with the desired length"),
                        time,
                        hear_id_type,
                        custom_volume_adjustments: custom_volume_adjustments
                            .try_into()
                            .expect("count is guaranteed to return a vec with the desired length"),
                        favorite_music_genre,
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes_with_music_genre_at_end(&self) -> impl Iterator<Item = u8> {
        iter::once(u8::from(self.is_enabled))
            .chain(self.volume_adjustment_bytes())
            .chain(self.time.to_be_bytes())
            .chain(iter::once(self.hear_id_type as u8))
            .chain(self.custom_volume_adjustment_bytes())
            .chain(self.favorite_music_genre.bytes())
    }

    pub fn volume_adjustment_bytes(&self) -> impl Iterator<Item = u8> {
        self.volume_adjustments
            .iter()
            .flat_map(|maybe_volume_adjustments| {
                maybe_volume_adjustments.map_or([0xFF; BANDS], |v| v.bytes())
            })
    }

    pub fn custom_volume_adjustment_bytes(&self) -> impl Iterator<Item = u8> {
        self.custom_volume_adjustments
            .iter()
            .flat_map(|maybe_volume_adjustments| {
                maybe_volume_adjustments.map_or([0xFF; BANDS], |v| v.bytes())
            })
    }
}

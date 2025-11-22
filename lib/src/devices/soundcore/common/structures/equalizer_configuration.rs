use std::array;

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::le_u16,
    sequence::pair,
};

use super::VolumeAdjustments;

pub type CommonEqualizerConfiguration<const CHANNELS: usize, const BANDS: usize> =
    EqualizerConfiguration<CHANNELS, BANDS, -120, 134, 1>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EqualizerConfiguration<
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    preset_id: u16,
    volume_adjustments:
        [VolumeAdjustments<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>; CHANNELS],
}

impl<
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> Default for EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>
{
    fn default() -> Self {
        Self::new(0, array::from_fn(|_| VolumeAdjustments::default()))
    }
}

impl<
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>
{
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "equalizer configuration",
            map(
                pair(
                    le_u16,
                    count(
                        VolumeAdjustments::<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>::take,
                        CHANNELS,
                    ),
                ),
                |(profile_id, volume_adjustments)| {
                    let volume_adjustments: [VolumeAdjustments<
                        BANDS,
                        MIN_VOLUME,
                        MAX_VOLUME,
                        FRACTION_DIGITS,
                    >; CHANNELS] = volume_adjustments
                        .try_into()
                        .expect("count vec is guaranteed to be the specified length");

                    Self::new(profile_id, volume_adjustments)
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        self.preset_id()
            .to_le_bytes()
            .into_iter()
            .chain(self.volume_adjustments.iter().flat_map(|v| v.bytes()))
    }

    pub fn new(
        preset_id: u16,
        volume_adjustments: [VolumeAdjustments<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>;
            CHANNELS],
    ) -> Self {
        Self {
            preset_id,
            volume_adjustments,
        }
    }

    pub fn preset_id(&self) -> u16 {
        self.preset_id
    }

    pub fn volume_adjustments_channel_1(
        &self,
    ) -> &VolumeAdjustments<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS> {
        &self.volume_adjustments[0]
    }

    pub fn volume_adjustments(
        &self,
    ) -> &[VolumeAdjustments<BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>; CHANNELS] {
        &self.volume_adjustments
    }

    pub fn channels(&self) -> usize {
        self.volume_adjustments.len()
    }

    pub fn bands(&self) -> usize {
        self.volume_adjustments_channel_1().adjustments().len()
    }
}

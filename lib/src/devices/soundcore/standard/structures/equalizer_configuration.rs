use std::array;

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::le_u16,
    sequence::pair,
};

use super::{VolumeAdjustments, preset_equalizer_profile::PresetEqualizerProfile};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EqualizerConfiguration<const CHANNELS: usize, const BANDS: usize> {
    preset_profile: Option<PresetEqualizerProfile>,
    volume_adjustments: [VolumeAdjustments<BANDS>; CHANNELS],
}

impl<const CHANNELS: usize, const BANDS: usize> Default
    for EqualizerConfiguration<CHANNELS, BANDS>
{
    fn default() -> Self {
        Self::new_from_preset_profile(
            PresetEqualizerProfile::SoundcoreSignature,
            array::from_fn(|_| vec![0; BANDS - 8]),
        )
    }
}

impl<const CHANNELS: usize, const BANDS: usize> EqualizerConfiguration<CHANNELS, BANDS> {
    pub const CUSTOM_PROFILE_ID: u16 = 0xfefe;

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "equalizer configuration",
            map(
                pair(le_u16, count(VolumeAdjustments::<BANDS>::take, CHANNELS)),
                |(profile_id, volume_adjustments)| {
                    let volume_adjustments: [VolumeAdjustments<BANDS>; CHANNELS] =
                        volume_adjustments
                            .try_into()
                            .expect("count vec is guaranteed to be the specified length");

                    match PresetEqualizerProfile::from_id(profile_id) {
                        Some(preset) => Self::new_from_preset_profile(
                            preset,
                            volume_adjustments.map(|channel| {
                                channel.adjustments().iter().skip(8).cloned().collect()
                            }),
                        ),
                        None => Self::new_custom_profile(volume_adjustments),
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        self.profile_id()
            .to_le_bytes()
            .into_iter()
            .chain(self.volume_adjustments.iter().flat_map(|v| v.bytes()))
    }

    /// The number of extra adjustments should be BANDS - 8. This function will panic otherwise.
    pub fn new_from_preset_profile(
        preset_profile: PresetEqualizerProfile,
        extra_adjustments: [Vec<i16>; CHANNELS],
    ) -> Self {
        let preset_adjustments = preset_profile.volume_adjustments();
        Self {
            preset_profile: Some(preset_profile),
            volume_adjustments: extra_adjustments.map(|channel_extras| {
                assert_eq!(
                    8 + channel_extras.len(),
                    BANDS,
                    "incorrect number of extra bands",
                );
                VolumeAdjustments::new(array::from_fn(|i| {
                    if i < preset_adjustments.adjustments().len() {
                        preset_adjustments.adjustments()[i]
                    } else {
                        channel_extras[i - preset_adjustments.adjustments().len()]
                    }
                }))
            }),
        }
    }

    pub fn new_custom_profile(volume_adjustments: [VolumeAdjustments<BANDS>; CHANNELS]) -> Self {
        Self {
            preset_profile: None,
            volume_adjustments,
        }
    }

    pub fn profile_id(&self) -> u16 {
        self.preset_profile
            .map(|preset_profile| preset_profile.id())
            .unwrap_or(Self::CUSTOM_PROFILE_ID)
    }

    pub fn preset_profile(&self) -> Option<PresetEqualizerProfile> {
        self.preset_profile
    }

    pub fn volume_adjustments_channel_1(&self) -> &VolumeAdjustments<BANDS> {
        &self.volume_adjustments[0]
    }

    pub fn volume_adjustments(&self) -> &[VolumeAdjustments<BANDS>; CHANNELS] {
        &self.volume_adjustments
    }

    pub fn channels(&self) -> usize {
        self.volume_adjustments.len()
    }

    pub fn bands(&self) -> usize {
        self.volume_adjustments_channel_1().adjustments().len()
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use super::*;

    #[test]
    fn new_from_preset_profile_with_no_extra_bands() {
        EqualizerConfiguration::<2, 8>::new_from_preset_profile(
            PresetEqualizerProfile::SoundcoreSignature,
            [Vec::new(), Vec::new()],
        );
    }

    #[test]
    fn new_from_preset_profile_with_correct_extra_bands() {
        EqualizerConfiguration::<2, 10>::new_from_preset_profile(
            PresetEqualizerProfile::SoundcoreSignature,
            [vec![1, 2], vec![3, 4]],
        );
    }

    #[test]
    #[should_panic]
    fn new_from_preset_profile_with_missing_extra_bands_fails() {
        EqualizerConfiguration::<2, 10>::new_from_preset_profile(
            PresetEqualizerProfile::SoundcoreSignature,
            [Vec::new(), Vec::new()],
        );
    }

    #[test]
    #[should_panic]
    fn new_from_preset_profile_with_not_enough_extra_bands_fails() {
        EqualizerConfiguration::<2, 10>::new_from_preset_profile(
            PresetEqualizerProfile::SoundcoreSignature,
            [vec![1], vec![2]],
        );
    }

    #[test]
    #[should_panic]
    fn new_from_preset_profile_with_too_many_extra_bands_fails() {
        EqualizerConfiguration::<2, 10>::new_from_preset_profile(
            PresetEqualizerProfile::SoundcoreSignature,
            [vec![1, 2, 3], vec![4, 5, 6]],
        );
    }

    #[test]
    fn take_with_extra_bands() {
        EqualizerConfiguration::<2, 10>::take::<VerboseError<_>>(&[0; 22]).unwrap();
    }
}

use std::array;

use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::le_u16,
    sequence::pair,
};

use super::{VolumeAdjustments, preset_equalizer_profile::PresetEqualizerProfile};

#[derive(Clone, Debug, PartialEq)]
pub struct EqualizerConfiguration<const CHANNELS: usize, const BANDS: usize> {
    preset_profile: Option<PresetEqualizerProfile>,
    volume_adjustments: [VolumeAdjustments<BANDS>; CHANNELS],
}

impl<const C: usize, const B: usize> Default for EqualizerConfiguration<C, B> {
    fn default() -> Self {
        Self::new_from_preset_profile(
            PresetEqualizerProfile::SoundcoreSignature,
            array::from_fn(|_| Vec::new()),
        )
    }
}

impl<const C: usize, const B: usize> EqualizerConfiguration<C, B> {
    pub const CUSTOM_PROFILE_ID: u16 = 0xfefe;

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "equalizer configuration",
            map(
                pair(le_u16, count(VolumeAdjustments::<B>::take, C)),
                |(profile_id, volume_adjustments)| {
                    let volume_adjustments: [VolumeAdjustments<B>; C] = volume_adjustments
                        .try_into()
                        .expect("count vec is guaranteed to be the specified length");

                    match PresetEqualizerProfile::from_id(profile_id) {
                        Some(preset) => Self::new_from_preset_profile(
                            preset,
                            volume_adjustments.map(|channel| {
                                channel.adjustments().iter().skip(8).cloned().collect()
                            }),
                        ),
                        None => EqualizerConfiguration::new_custom_profile(volume_adjustments),
                    }
                },
            ),
        )(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        self.profile_id()
            .to_le_bytes()
            .into_iter()
            .chain(self.volume_adjustments.iter().flat_map(|v| v.bytes()))
    }

    pub fn new_from_preset_profile(
        preset_profile: PresetEqualizerProfile,
        extra_adjustments: [Vec<i16>; C],
    ) -> Self {
        let preset_adjustments = preset_profile.volume_adjustments();
        Self {
            preset_profile: Some(preset_profile),
            volume_adjustments: array::from_fn(|i| {
                VolumeAdjustments::new(array::from_fn(|j| {
                    if j < preset_adjustments.adjustments().len() {
                        preset_adjustments.adjustments()[j]
                    } else {
                        extra_adjustments[i][j - preset_adjustments.adjustments().len()]
                    }
                }))
            }),
        }
    }

    pub fn new_custom_profile(volume_adjustments: [VolumeAdjustments<B>; C]) -> Self {
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

    pub fn volume_adjustments_channel_1(&self) -> &VolumeAdjustments<B> {
        &self.volume_adjustments[0]
    }

    pub fn volume_adjustments(&self) -> &[VolumeAdjustments<B>; C] {
        &self.volume_adjustments
    }

    pub fn channels(&self) -> usize {
        self.volume_adjustments.len()
    }

    pub fn bands(&self) -> usize {
        self.volume_adjustments_channel_1().adjustments().len()
    }
}

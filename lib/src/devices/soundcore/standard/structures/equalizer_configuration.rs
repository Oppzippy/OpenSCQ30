use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::le_u16,
    sequence::pair,
};
use serde::{Deserialize, Serialize};

use super::{VolumeAdjustments, preset_equalizer_profile::PresetEqualizerProfile};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EqualizerConfiguration {
    preset_profile: Option<PresetEqualizerProfile>,
    volume_adjustments: Vec<VolumeAdjustments>,
}

impl Default for EqualizerConfiguration {
    fn default() -> Self {
        Self::new_from_preset_profile(1, PresetEqualizerProfile::SoundcoreSignature, Vec::new())
    }
}

impl EqualizerConfiguration {
    pub const CUSTOM_PROFILE_ID: u16 = 0xfefe;

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        num_channels: usize,
        num_bands: usize,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], EqualizerConfiguration, E> {
        move |input| {
            context(
                "equalizer configuration",
                map(
                    pair(
                        le_u16,
                        count(VolumeAdjustments::take(num_bands), num_channels),
                    ),
                    |(profile_id, volume_adjustments)| match PresetEqualizerProfile::from_id(
                        profile_id,
                    ) {
                        Some(preset) => EqualizerConfiguration::new_from_preset_profile(
                            num_channels,
                            preset,
                            volume_adjustments
                                .into_iter()
                                .map(|channel| {
                                    channel.adjustments().iter().skip(8).cloned().collect()
                                })
                                .collect(),
                        ),
                        None => EqualizerConfiguration::new_custom_profile(volume_adjustments),
                    },
                ),
            )(input)
        }
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        self.profile_id()
            .to_le_bytes()
            .into_iter()
            .chain(self.volume_adjustments.iter().map(|v| v.bytes()).flatten())
    }

    pub fn new_from_preset_profile(
        num_channels: usize,
        preset_profile: PresetEqualizerProfile,
        extra_adjustments: Vec<Vec<i16>>,
    ) -> Self {
        assert!(
            num_channels == extra_adjustments.len() || extra_adjustments.is_empty(),
            "num_channels ({num_channels}) should be consistent with extra_adjustments's ({}) number of channels",
            extra_adjustments.len()
        );
        Self {
            preset_profile: Some(preset_profile),
            volume_adjustments: (0..num_channels)
                .into_iter()
                .map(|i| {
                    VolumeAdjustments::new(
                        preset_profile
                            .volume_adjustments()
                            .adjustments()
                            .iter()
                            .chain(extra_adjustments.get(i).into_iter().flatten())
                            .cloned()
                            .collect(),
                    )
                    .expect("all preset profiles should be valid")
                })
                .collect(),
        }
    }

    pub fn new_custom_profile(volume_adjustments: Vec<VolumeAdjustments>) -> Self {
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

    pub fn volume_adjustments_channel_1(&self) -> &VolumeAdjustments {
        &self.volume_adjustments.first().unwrap()
    }

    pub fn volume_adjustments(&self) -> &[VolumeAdjustments] {
        &self.volume_adjustments
    }

    pub fn channels(&self) -> usize {
        self.volume_adjustments.len()
    }

    pub fn bands(&self) -> usize {
        self.volume_adjustments_channel_1().adjustments().len()
    }
}

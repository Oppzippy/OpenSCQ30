use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u16,
    sequence::pair,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{VolumeAdjustments, preset_equalizer_profile::PresetEqualizerProfile};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct EqualizerConfiguration {
    preset_profile: Option<PresetEqualizerProfile>,
    volume_adjustments: VolumeAdjustments,
}

impl Default for EqualizerConfiguration {
    fn default() -> Self {
        Self::new_from_preset_profile(PresetEqualizerProfile::SoundcoreSignature)
    }
}

impl EqualizerConfiguration {
    pub const CUSTOM_PROFILE_ID: u16 = 0xfefe;

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        num_bands: usize,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], EqualizerConfiguration, E> {
        move |input| {
            context(
                "equalizer configuration",
                map(
                    pair(le_u16, VolumeAdjustments::take(num_bands)),
                    |(profile_id, volume_adjustments)| {
                        PresetEqualizerProfile::from_id(profile_id)
                            .map(EqualizerConfiguration::new_from_preset_profile)
                            .unwrap_or(EqualizerConfiguration::new_custom_profile(
                                volume_adjustments,
                            ))
                    },
                ),
            )(input)
        }
    }

    pub fn new_from_preset_profile(preset_profile: PresetEqualizerProfile) -> Self {
        Self {
            preset_profile: Some(preset_profile),
            volume_adjustments: preset_profile.volume_adjustments(),
        }
    }

    pub fn new_custom_profile(volume_adjustments: VolumeAdjustments) -> Self {
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

    pub fn volume_adjustments(&self) -> &VolumeAdjustments {
        &self.volume_adjustments
    }
}

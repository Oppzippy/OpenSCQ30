#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{preset_equalizer_profile::PresetEqualizerProfile, VolumeAdjustments};

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

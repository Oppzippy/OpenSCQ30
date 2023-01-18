use crate::packets::structures::equalizer_band_offsets::EqualizerBandOffsets;

use super::preset_equalizer_profile::PresetEqualizerProfile;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct EqualizerConfiguration {
    preset_profile: Option<PresetEqualizerProfile>,
    band_offsets: EqualizerBandOffsets,
}

impl EqualizerConfiguration {
    pub const CUSTOM_PROFILE_ID: u16 = 0xfefe;

    pub fn new_from_preset_profile(preset_profile: PresetEqualizerProfile) -> Self {
        Self {
            preset_profile: Some(preset_profile),
            band_offsets: preset_profile.band_offsets(),
        }
    }

    pub fn new_custom_profile(band_offsets: EqualizerBandOffsets) -> Self {
        Self {
            preset_profile: None,
            band_offsets,
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

    pub fn band_offsets(&self) -> EqualizerBandOffsets {
        self.band_offsets
    }
}

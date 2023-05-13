use openscq30_lib::packets::structures;
use wasm_bindgen::prelude::wasm_bindgen;

use super::{PresetEqualizerProfile, VolumeAdjustments};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[wasm_bindgen]
pub struct EqualizerConfiguration {
    inner: structures::EqualizerConfiguration,
}

#[wasm_bindgen]
impl EqualizerConfiguration {
    #[wasm_bindgen(js_name = "fromPresetProfile")]
    pub fn new_from_preset_profile(
        preset_profile: PresetEqualizerProfile,
    ) -> EqualizerConfiguration {
        Self {
            inner: structures::EqualizerConfiguration::new_from_preset_profile(
                preset_profile.into(),
            ),
        }
    }

    #[wasm_bindgen(js_name = "fromCustomProfile")]
    pub fn new_custom_profile(volume_adjustments: &VolumeAdjustments) -> EqualizerConfiguration {
        Self {
            inner: structures::EqualizerConfiguration::new_custom_profile(
                volume_adjustments.to_owned().into(),
            ),
        }
    }

    #[wasm_bindgen(getter = profileId)]
    pub fn profile_id(&self) -> i32 {
        self.inner.profile_id().into()
    }

    #[wasm_bindgen(getter = presetProfile)]
    pub fn preset_profile(&self) -> Option<PresetEqualizerProfile> {
        self.inner.preset_profile().map(|profile| profile.into())
    }

    #[wasm_bindgen(getter = volumeAdjustments)]
    pub fn volume_adjustments(&self) -> VolumeAdjustments {
        self.inner.volume_adjustments().into()
    }
}

impl From<structures::EqualizerConfiguration> for EqualizerConfiguration {
    fn from(value: structures::EqualizerConfiguration) -> Self {
        Self { inner: value }
    }
}

impl From<EqualizerConfiguration> for structures::EqualizerConfiguration {
    fn from(value: EqualizerConfiguration) -> Self {
        value.inner
    }
}

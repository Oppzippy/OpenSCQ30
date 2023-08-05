use std::str::FromStr;

use openscq30_lib::packets::structures::{
    EqualizerConfiguration, PresetEqualizerProfile, VolumeAdjustments,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct EqualizerHelper {}

#[wasm_bindgen]
impl EqualizerHelper {
    #[wasm_bindgen(getter = MIN_VOLUME)]
    pub fn min_volume() -> i8 {
        VolumeAdjustments::MIN_VOLUME
    }

    #[wasm_bindgen(getter = MAX_VOLUME)]
    pub fn max_volume() -> i8 {
        VolumeAdjustments::MAX_VOLUME
    }

    #[wasm_bindgen(js_name = "getPresetProfileVolumeAdjustments")]
    pub fn preset_profile_volume_adjustments(profile_name: String) -> Result<Vec<i8>, String> {
        let preset_profile =
            PresetEqualizerProfile::from_str(&profile_name).map_err(|err| format!("{err:?}"))?;
        let equalizer_configuration =
            EqualizerConfiguration::new_from_preset_profile(preset_profile);
        Ok(equalizer_configuration
            .volume_adjustments()
            .adjustments()
            .to_vec())
    }
}

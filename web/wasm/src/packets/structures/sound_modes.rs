use super::{AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, TransparencyMode};
use openscq30_lib::packets::structures::SoundModes as LibSoundModes;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub transparency_mode: TransparencyMode,
    pub custom_noise_canceling: CustomNoiseCanceling,
}

impl From<LibSoundModes> for SoundModes {
    fn from(value: LibSoundModes) -> Self {
        Self {
            ambient_sound_mode: value.ambient_sound_mode.into(),
            noise_canceling_mode: value.noise_canceling_mode.into(),
            custom_noise_canceling: value.custom_noise_canceling.into(),
            transparency_mode: value.transparency_mode.into(),
        }
    }
}

use wasm_bindgen::prelude::wasm_bindgen;

use crate::packets::structures::{AmbientSoundMode, NoiseCancelingMode};
use openscq30_lib::packets::inbound::SoundModeUpdatePacket as LibSoundModeUpdatePacket;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub struct SoundModeUpdatePacket(LibSoundModeUpdatePacket);

#[wasm_bindgen]
impl SoundModeUpdatePacket {
    #[wasm_bindgen(getter = ambientSoundMode)]
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.0.ambient_sound_mode().into()
    }

    #[wasm_bindgen(getter = noiseCancelingMode)]
    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.0.noise_canceling_mode().into()
    }
}

impl From<LibSoundModeUpdatePacket> for SoundModeUpdatePacket {
    fn from(packet: LibSoundModeUpdatePacket) -> Self {
        Self(packet)
    }
}

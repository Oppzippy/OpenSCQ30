use wasm_bindgen::prelude::wasm_bindgen;

use crate::packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode};
use openscq30_lib::packets::inbound::StateUpdatePacket as LibStateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub struct StateUpdatePacket(LibStateUpdatePacket);

#[wasm_bindgen]
impl StateUpdatePacket {
    #[wasm_bindgen(getter = ambientSoundMode)]
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.0.ambient_sound_mode().into()
    }

    #[wasm_bindgen(getter = noiseCancelingMode)]
    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.0.noise_canceling_mode().into()
    }

    #[wasm_bindgen(getter = equalizerConfiguration)]
    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.0.equalizer_configuration().into()
    }
}

impl From<LibStateUpdatePacket> for StateUpdatePacket {
    fn from(packet: LibStateUpdatePacket) -> Self {
        Self(packet)
    }
}

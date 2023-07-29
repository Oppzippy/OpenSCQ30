use wasm_bindgen::prelude::wasm_bindgen;

use crate::{EqualizerConfiguration, SoundModes};
use openscq30_lib::packets::inbound::state_update_packet::StateUpdatePacket as LibStateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub struct StateUpdatePacket(LibStateUpdatePacket);

#[wasm_bindgen]
impl StateUpdatePacket {
    #[wasm_bindgen(getter = soundModes)]
    pub fn sound_modes(&self) -> Option<SoundModes> {
        self.0.sound_modes.map(Into::into)
    }

    #[wasm_bindgen(getter = equalizerConfiguration)]
    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.0.equalizer_configuration.into()
    }
}

impl From<LibStateUpdatePacket> for StateUpdatePacket {
    fn from(packet: LibStateUpdatePacket) -> Self {
        Self(packet)
    }
}

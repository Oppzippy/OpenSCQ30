use wasm_bindgen::prelude::wasm_bindgen;

use crate::packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub struct StateUpdatePacket {
    packet: Option<openscq30_lib::packets::inbound::StateUpdatePacket>,
}

#[wasm_bindgen]
impl StateUpdatePacket {
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<StateUpdatePacket>, String> {
        Ok(
            openscq30_lib::packets::inbound::StateUpdatePacket::new(bytes)
                .map(|packet| packet.into()),
        )
    }

    #[wasm_bindgen(getter = ambientSoundMode)]
    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.packet.unwrap().ambient_sound_mode().into()
    }

    #[wasm_bindgen(getter = noiseCancelingMode)]
    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.packet.unwrap().noise_canceling_mode().into()
    }

    #[wasm_bindgen(getter = equalizerConfiguration)]
    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.packet.unwrap().equalizer_configuration().into()
    }
}

impl From<openscq30_lib::packets::inbound::StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: openscq30_lib::packets::inbound::StateUpdatePacket) -> Self {
        Self {
            packet: Some(packet),
        }
    }
}

impl From<StateUpdatePacket> for openscq30_lib::packets::inbound::StateUpdatePacket {
    fn from(value: StateUpdatePacket) -> Self {
        value.packet.unwrap()
    }
}

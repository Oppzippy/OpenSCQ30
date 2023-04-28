use wasm_bindgen::prelude::wasm_bindgen;

use crate::packets::structures::{AmbientSoundMode, NoiseCancelingMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub struct AmbientSoundModeUpdatePacket {
    packet: Option<openscq30_lib::packets::inbound::AmbientSoundModeUpdatePacket>,
}

#[wasm_bindgen]
impl AmbientSoundModeUpdatePacket {
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<AmbientSoundModeUpdatePacket>, String> {
        Ok(
            openscq30_lib::packets::inbound::AmbientSoundModeUpdatePacket::new(bytes)
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
}

impl From<openscq30_lib::packets::inbound::AmbientSoundModeUpdatePacket>
    for AmbientSoundModeUpdatePacket
{
    fn from(packet: openscq30_lib::packets::inbound::AmbientSoundModeUpdatePacket) -> Self {
        Self {
            packet: Some(packet),
        }
    }
}

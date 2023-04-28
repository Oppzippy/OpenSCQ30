use openscq30_lib::packets::outbound::OutboundPacket as _;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::packets::structures::{AmbientSoundMode, NoiseCancelingMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub struct SetAmbientSoundModePacket {
    packet: openscq30_lib::packets::outbound::SetAmbientSoundModePacket,
}

#[wasm_bindgen]
impl SetAmbientSoundModePacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        ambient_sound_mode: AmbientSoundMode,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> SetAmbientSoundModePacket {
        Self {
            packet: openscq30_lib::packets::outbound::SetAmbientSoundModePacket::new(
                ambient_sound_mode.to_owned().into(),
                noise_canceling_mode.to_owned().into(),
            ),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.packet.bytes()
    }
}

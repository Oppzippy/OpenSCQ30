use openscq30_lib::packets::outbound::OutboundPacket as _;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::packets::structures::{AmbientSoundMode, NoiseCancelingMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub struct SetSoundModePacket {
    packet: openscq30_lib::packets::outbound::SetSoundModePacket,
}

#[wasm_bindgen]
impl SetSoundModePacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        ambient_sound_mode: AmbientSoundMode,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> SetSoundModePacket {
        Self {
            packet: openscq30_lib::packets::outbound::SetSoundModePacket {
                ambient_sound_mode: ambient_sound_mode.to_owned().into(),
                noise_canceling_mode: noise_canceling_mode.to_owned().into(),
                transparency_mode: Default::default(),
                custom_noise_canceling: Default::default(),
            },
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.packet.bytes()
    }
}

use openscq30_lib::packets::outbound::OutboundPacket as _;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::packets::structures::EqualizerConfiguration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub struct SetEqualizerPacket {
    packet: openscq30_lib::packets::outbound::SetEqualizerPacket,
}

#[wasm_bindgen]
impl SetEqualizerPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(configuration: &EqualizerConfiguration) -> SetEqualizerPacket {
        Self {
            packet: openscq30_lib::packets::outbound::SetEqualizerPacket::new(
                configuration.to_owned().into(),
            ),
        }
    }
    pub fn bytes(&self) -> Vec<u8> {
        self.packet.bytes()
    }
}

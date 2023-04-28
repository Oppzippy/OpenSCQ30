use openscq30_lib::packets::outbound::OutboundPacket as _;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[wasm_bindgen]
pub struct RequestStatePacket {
    packet: openscq30_lib::packets::outbound::RequestStatePacket,
}

#[wasm_bindgen]
impl RequestStatePacket {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RequestStatePacket {
        Self {
            packet: openscq30_lib::packets::outbound::RequestStatePacket::new(),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.packet.bytes()
    }
}

use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[wasm_bindgen]
pub struct SetEqualizerOkPacket {
    packet: Option<openscq30_lib::packets::inbound::SetEqualizerOkPacket>,
}

#[wasm_bindgen]
impl SetEqualizerOkPacket {
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<SetEqualizerOkPacket>, String> {
        Ok(
            openscq30_lib::packets::inbound::SetEqualizerOkPacket::new(bytes)
                .map(|packet| packet.into()),
        )
    }
}

impl From<openscq30_lib::packets::inbound::SetEqualizerOkPacket> for SetEqualizerOkPacket {
    fn from(packet: openscq30_lib::packets::inbound::SetEqualizerOkPacket) -> Self {
        Self {
            packet: Some(packet),
        }
    }
}

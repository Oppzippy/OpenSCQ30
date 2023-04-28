use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[wasm_bindgen]
pub struct SetAmbientModeOkPacket {
    packet: Option<openscq30_lib::packets::inbound::SetAmbientModeOkPacket>,
}

#[wasm_bindgen]
impl SetAmbientModeOkPacket {
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<SetAmbientModeOkPacket>, String> {
        Ok(
            openscq30_lib::packets::inbound::SetAmbientModeOkPacket::new(bytes)
                .map(|packet| packet.into()),
        )
    }
}

impl From<openscq30_lib::packets::inbound::SetAmbientModeOkPacket> for SetAmbientModeOkPacket {
    fn from(packet: openscq30_lib::packets::inbound::SetAmbientModeOkPacket) -> Self {
        Self {
            packet: Some(packet),
        }
    }
}

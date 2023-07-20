use openscq30_lib::packets::inbound::SetEqualizerOkPacket as LibSetEqualizerOkPacket;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[wasm_bindgen]
pub struct SetEqualizerOkPacket(LibSetEqualizerOkPacket);

#[wasm_bindgen]
impl SetEqualizerOkPacket {}

impl From<LibSetEqualizerOkPacket> for SetEqualizerOkPacket {
    fn from(packet: LibSetEqualizerOkPacket) -> Self {
        Self(packet)
    }
}

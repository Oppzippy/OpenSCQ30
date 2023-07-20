use openscq30_lib::packets::inbound::SetSoundModeOkPacket as LibSetSoundModeOkPacket;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[wasm_bindgen]
pub struct SetSoundModeOkPacket(LibSetSoundModeOkPacket);

#[wasm_bindgen]
impl SetSoundModeOkPacket {}

impl From<LibSetSoundModeOkPacket> for SetSoundModeOkPacket {
    fn from(packet: LibSetSoundModeOkPacket) -> Self {
        Self(packet)
    }
}

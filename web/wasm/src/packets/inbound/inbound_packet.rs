use openscq30_lib::packets::inbound::InboundPacket as LibInboundPacket;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{SetEqualizerOkPacket, SetSoundModeOkPacket, SoundModeUpdatePacket, StateUpdatePacket};

#[wasm_bindgen]
pub struct InboundPacket(LibInboundPacket);

#[wasm_bindgen]
impl InboundPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &[u8]) -> Result<InboundPacket, String> {
        LibInboundPacket::new(input)
            .map(Self)
            .map_err(|err| err.to_string())
    }

    #[wasm_bindgen(getter = ambientSoundModeUpdate)]
    pub fn sound_mode_update(&self) -> Option<SoundModeUpdatePacket> {
        if let LibInboundPacket::SoundModeUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[wasm_bindgen(getter = setSoundModeOk)]
    pub fn set_sound_mode_ok(&self) -> Option<SetSoundModeOkPacket> {
        if let LibInboundPacket::SetSoundModeOk(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[wasm_bindgen(getter = setEqualizerOk)]
    pub fn set_equalizer_ok(&self) -> Option<SetEqualizerOkPacket> {
        if let LibInboundPacket::SetEqualizerOk(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[wasm_bindgen(getter = stateUpdate)]
    pub fn state_update(&self) -> Option<StateUpdatePacket> {
        if let LibInboundPacket::StateUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }
}

mod packets;
mod soundcore_device_utils;
use wasm_bindgen::prelude::wasm_bindgen;

pub use crate::packets::inbound::*;
pub use crate::packets::outbound::*;
pub use crate::packets::structures::*;
pub use crate::soundcore_device_utils::*;

pub struct Init {}

#[wasm_bindgen]
impl Init {
    pub fn logging() {}
}

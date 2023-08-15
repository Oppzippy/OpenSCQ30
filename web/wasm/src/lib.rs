mod device;
mod device_feature_flags;
mod equalizer_helper;
mod soundcore_device_utils;
pub mod web_bluetooth_connection;

pub use device::*;
pub use device_feature_flags::*;
pub use equalizer_helper::*;
pub use soundcore_device_utils::*;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn initialize() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}

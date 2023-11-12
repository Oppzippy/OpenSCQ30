mod device;
mod equalizer_helper;
mod jsvalue_error;
mod soundcore_device_utils;
pub mod web_bluetooth_connection;

pub use device::*;
pub use equalizer_helper::*;
pub use jsvalue_error::*;
use openscq30_lib::devices::standard::state::DeviceState;
pub use soundcore_device_utils::*;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn initialize() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}

#[wasm_bindgen]
pub struct WasmTest;
#[wasm_bindgen]
impl WasmTest {
    #[wasm_bindgen(js_name = "deserializeAndReserializeForTests")]
    pub fn deserialize_and_reserialize_for_tests(input: String) -> Result<String, String> {
        let state =
            serde_json::from_str::<DeviceState>(&input).map_err(|err| format!("{err:?}"))?;
        serde_json::to_string(&state).map_err(|err| format!("{err:?}"))
    }
}

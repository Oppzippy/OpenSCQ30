use openscq30_lib::device_utils;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct SoundcoreDeviceUtils {}

#[wasm_bindgen]
impl SoundcoreDeviceUtils {
    #[wasm_bindgen(js_name = "getServiceUuid")]
    pub fn service_uuid() -> String {
        device_utils::SERVICE_UUID.to_string()
    }

    #[wasm_bindgen(js_name = "getReadCharacteristicUuid")]
    pub fn read_characteristic_uuid() -> String {
        device_utils::READ_CHARACTERISTIC_UUID.to_string()
    }

    #[wasm_bindgen(js_name = "getWriteCharacteristicUuid")]
    pub fn write_characteristic_uuid() -> String {
        device_utils::WRITE_CHARACTERISTIC_UUID.to_string()
    }
}

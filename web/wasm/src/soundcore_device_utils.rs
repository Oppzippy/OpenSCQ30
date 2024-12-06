use openscq30_lib::device_utils;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(typescript_custom_section)]
const MAC_ADDRESS_PREFIXES: &'static str = r#"
type MacAddressPrefixes = number[][]
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "MacAddressPrefixes")]
    pub type MacAddressPrefixes;
}

#[wasm_bindgen]
pub struct SoundcoreDeviceUtils {}

#[derive(Serialize, Deserialize)]
pub struct Container(Vec<String>);

#[wasm_bindgen]
impl SoundcoreDeviceUtils {
    // WebBluetooth doesn't allow iterating over all services, only ones that we specify.
    // Since I believe the uuids are sequential, we can just list all of the ones around a known uuid.
    #[wasm_bindgen(js_name = "getServiceUuids")]
    pub fn service_uuids() -> Result<JsValue, JsValue> {
        let uuids = device_utils::service_uuids()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        Ok(serde_wasm_bindgen::to_value(&uuids)?)
    }

    #[wasm_bindgen(js_name = "getReadCharacteristicUuid")]
    pub fn read_characteristic_uuid() -> String {
        device_utils::READ_CHARACTERISTIC_UUID.to_string()
    }

    #[wasm_bindgen(js_name = "getWriteCharacteristicUuid")]
    pub fn write_characteristic_uuid() -> String {
        device_utils::WRITE_CHARACTERISTIC_UUID.to_string()
    }

    #[wasm_bindgen(js_name = "getMacAddressPrefixes")]
    pub fn get_mac_address_prefixes() -> Result<MacAddressPrefixes, JsValue> {
        let prefixes = device_utils::soundcore_mac_address_prefixes()
            .iter()
            .map(|prefix| prefix.to_vec())
            .collect::<Vec<_>>();
        Ok(serde_wasm_bindgen::to_value(&prefixes)?.into())
    }
}

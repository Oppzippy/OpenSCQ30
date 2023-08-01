use openscq30_lib::device_utils;
use serde::{Deserialize, Serialize};
use static_assertions::const_assert;
use uuid::Uuid;
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

// outside of wasm_bindgen for tests
fn service_uuids() -> Vec<String> {
    const KNOWN_UUID: u128 = device_utils::SERVICE_UUID.as_u128();
    const GENERIC_PART: u128 =
        device_utils::SERVICE_UUID.as_u128() & device_utils::SERVICE_UUID_MASK.as_u128();

    // keep the two bytes of the device specific part, shift everything else to the right out of existence
    const DEVICE_SPECIFIC_PART_SHIFT: i32 = 128 - 16;
    const DEVICE_SPECIFIC_PART_CENTER: u128 =
        (KNOWN_UUID ^ GENERIC_PART) >> DEVICE_SPECIFIC_PART_SHIFT;

    const HALF_RANGE: u128 = 32;
    const MIN: u128 = DEVICE_SPECIFIC_PART_CENTER.wrapping_sub(HALF_RANGE);
    const MAX: u128 = DEVICE_SPECIFIC_PART_CENTER + HALF_RANGE;
    const_assert!(MIN < MAX); // should not underoverflow
    const_assert!(MAX < 0xFFFF); // should not overflow

    let surrounding_uuids = (MIN..MAX)
        .map(|device_specific_part| {
            let uuid = (device_specific_part << DEVICE_SPECIFIC_PART_SHIFT) | GENERIC_PART;
            Uuid::from_u128(uuid).to_string()
        })
        .collect::<Vec<_>>();
    surrounding_uuids
}

#[wasm_bindgen]
impl SoundcoreDeviceUtils {
    // WebBluetooth doesn't allow iterating over all services, only ones that we specify.
    // Since I believe the uuids are sequential, we can just list all of the ones around a known uuid.
    #[wasm_bindgen(js_name = "getServiceUuids")]
    pub fn service_uuids() -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&service_uuids())?)
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

#[cfg(test)]
mod tests {
    use crate::soundcore_device_utils;

    #[test]
    fn test_service_uuids_are_in_correct_range() {
        let service_uuids = soundcore_device_utils::service_uuids();
        assert_eq!(
            true,
            service_uuids.contains(&"0100f5da-0000-1000-8000-00805f9b34fb".into())
        );
        assert_eq!(
            true,
            service_uuids.contains(&"011cf5da-0000-1000-8000-00805f9b34fb".into())
        );
        assert_eq!(
            true,
            service_uuids.contains(&"0120f5da-0000-1000-8000-00805f9b34fb".into())
        );
    }
}

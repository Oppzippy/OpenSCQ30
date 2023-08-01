use std::str::FromStr;

use macaddr::MacAddr;
use openscq30_lib::device_utils::{self, is_soundcore_service_uuid};
use rifgen::rifgen_attr::generate_interface;
use uuid::Uuid;

pub struct SoundcoreDeviceUtils {}

impl SoundcoreDeviceUtils {
    #[generate_interface]
    pub fn is_mac_address_soundcore_device(mac_address: &str) -> bool {
        match MacAddr::from_str(mac_address) {
            Ok(MacAddr::V6(parsed_address)) => {
                device_utils::is_mac_address_soundcore_device(parsed_address)
            }
            _ => false,
        }
    }

    #[generate_interface]
    pub fn is_soundcore_service_uuid(
        most_significant_bits: i64,
        least_significant_bits: i64,
    ) -> bool {
        // Java doesn't have u64, so take them as i64 and convert to u64, which is a noop
        let uuid = Uuid::from_u64_pair(most_significant_bits as u64, least_significant_bits as u64);
        is_soundcore_service_uuid(&uuid)
    }

    #[generate_interface]
    pub fn read_characteristic_uuid() -> String {
        device_utils::READ_CHARACTERISTIC_UUID.to_string()
    }

    #[generate_interface]
    pub fn write_characteristic_uuid() -> String {
        device_utils::WRITE_CHARACTERISTIC_UUID.to_string()
    }
}

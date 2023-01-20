use std::str::FromStr;

use macaddr::MacAddr;
use openscq30_lib::soundcore_device_utils;
use rifgen::rifgen_attr::generate_interface;

pub struct SoundcoreDeviceUtils {}

impl SoundcoreDeviceUtils {
    #[generate_interface]
    pub fn is_mac_address_soundcore_device(mac_address: &str) -> bool {
        match MacAddr::from_str(mac_address) {
            Ok(MacAddr::V6(parsed_address)) => {
                soundcore_device_utils::is_mac_address_soundcore_device(parsed_address.into_array())
            }
            _ => false,
        }
    }

    #[generate_interface]
    pub fn service_uuid() -> String {
        soundcore_device_utils::SERVICE_UUID.to_string()
    }

    #[generate_interface]
    pub fn read_characteristic_uuid() -> String {
        soundcore_device_utils::READ_CHARACTERISTIC_UUID.to_string()
    }

    #[generate_interface]
    pub fn write_characteristic_uuid() -> String {
        soundcore_device_utils::WRITE_CHARACTERISTIC_UUID.to_string()
    }
}

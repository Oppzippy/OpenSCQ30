use macaddr::MacAddr6;
use uuid::Uuid;

// The devices have the same UUID except the first two bytes. I assume one device was chosen with an initial value,
// and then the first two bytes increment by one for each device going from there. Unsure of the initial value or
// the number of devices in existence.
pub const SERVICE_UUID: Uuid = uuid::uuid!("011cf5da-0000-1000-8000-00805f9b34fb");
pub const SERVICE_UUID_MASK: Uuid = uuid::uuid!("0000FFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF");

pub const WRITE_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("00007777-0000-1000-8000-00805f9b34fb");
pub const READ_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("00008888-0000-1000-8000-00805f9b34fb");

// All mac address prefixes owned by Fantasia Trading LLC should be listed here.
// See: http://standards-oui.ieee.org/oui/oui.csv
const MAC_ADDRESS_PREFIXES: [[u8; 3]; 4] = [
    [0xAC, 0x12, 0x2F],
    [0xE8, 0xEE, 0xCC],
    [0xA4, 0x77, 0x58],
    [0xA0, 0xE9, 0xDB],
];
pub fn soundcore_mac_address_prefixes() -> &'static [[u8; 3]] {
    &MAC_ADDRESS_PREFIXES
}

pub fn is_mac_address_soundcore_device(mac_address: MacAddr6) -> bool {
    soundcore_mac_address_prefixes()
        .iter()
        .any(|range| mac_address.as_bytes().starts_with(range))
}

pub fn is_soundcore_service_uuid(uuid: &Uuid) -> bool {
    let mask = SERVICE_UUID_MASK.as_u128();
    uuid.as_u128() & mask == SERVICE_UUID.as_u128() & mask
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use uuid::Uuid;

    use crate::device_utils::is_mac_address_soundcore_device;

    use super::is_soundcore_service_uuid;

    #[test]
    fn test_soundcore_device_mac_address() {
        let mac_address = [0xAC, 0x12, 0x2F, 0x00, 0x00, 0x00].into();
        assert_eq!(true, is_mac_address_soundcore_device(mac_address));
    }

    #[test]
    fn test_not_soundcore_device_mac_address() {
        let mac_address = [0xAC, 0x00, 0x00, 0x00, 0x00, 0x00].into();
        assert_eq!(false, is_mac_address_soundcore_device(mac_address));
    }

    #[test]
    fn test_valid_service_uuid() {
        let uuid = Uuid::from_str("1234f5da-0000-1000-8000-00805f9b34fb").unwrap();
        assert_eq!(true, is_soundcore_service_uuid(&uuid));
    }

    #[test]
    fn test_invalid_service_uuid() {
        let uuid = Uuid::from_str("123455da-0000-1000-8000-00805f9b34fb").unwrap();
        assert_eq!(false, is_soundcore_service_uuid(&uuid));
    }
}

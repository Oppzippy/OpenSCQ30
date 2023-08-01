use macaddr::MacAddr6;
use uuid::Uuid;

pub const SERVICE_UUID: Uuid = uuid::uuid!("011cf5da-0000-1000-8000-00805f9b34fb");
pub const WRITE_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("00007777-0000-1000-8000-00805f9b34fb");
pub const READ_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("00008888-0000-1000-8000-00805f9b34fb");

// All mac address prefixes owned by Fantasia Trading LLC should be listed here.
// See: http://standards-oui.ieee.org/oui/oui.csv
const MAC_ADDRESS_PREFIXES: [[u8; 3]; 2] = [[0xAC, 0x12, 0x2F], [0xE8, 0xEE, 0xCC]];
pub fn soundcore_mac_address_prefixes() -> &'static [[u8; 3]] {
    &MAC_ADDRESS_PREFIXES
}

pub fn is_mac_address_soundcore_device(mac_address: MacAddr6) -> bool {
    soundcore_mac_address_prefixes()
        .iter()
        .any(|range| mac_address.as_bytes().starts_with(range))
}

#[cfg(test)]
mod tests {
    use crate::device_utils::is_mac_address_soundcore_device;

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
}

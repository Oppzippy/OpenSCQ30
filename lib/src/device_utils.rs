use macaddr::MacAddr6;
use static_assertions::const_assert;
use uuid::{Uuid, uuid};

pub const RFCOMM_UUID: Uuid = uuid!("00001101-0000-1000-8000-00805f9b34fb");
pub const VENDOR_RFCOMM_UUID: Uuid = uuid!("0cf12d31-fac3-4553-bd80-d6832e7b0000");
pub const VENDOR_RFCOMM_UUID_MASK: Uuid = uuid!("FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFF0000");

pub fn is_soundcore_vendor_rfcomm_uuid(uuid: &Uuid) -> bool {
    let mask = VENDOR_RFCOMM_UUID_MASK.as_u128();
    uuid.as_u128() & mask == VENDOR_RFCOMM_UUID.as_u128() & mask
}

// The devices have the same UUID except the first two bytes. I assume one device was chosen with an initial value,
// and then the first two bytes increment by one for each device going from there. Unsure of the initial value or
// the number of devices in existence.
pub const SERVICE_UUID: Uuid = uuid!("011cf5da-0000-1000-8000-00805f9b34fb");
pub const SERVICE_UUID_MASK: Uuid = uuid!("0000FFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF");

pub const WRITE_CHARACTERISTIC_UUID: Uuid = uuid!("00007777-0000-1000-8000-00805f9b34fb");
pub const READ_CHARACTERISTIC_UUID: Uuid = uuid!("00008888-0000-1000-8000-00805f9b34fb");

pub fn is_soundcore_service_uuid(uuid: &Uuid) -> bool {
    let mask = SERVICE_UUID_MASK.as_u128();
    uuid.as_u128() & mask == SERVICE_UUID.as_u128() & mask
}

/// Returns the surrounding service uuids of a known valid one.
/// This should hopefully cover all devices, and if not, the range can be increased.
pub fn service_uuids() -> Vec<Uuid> {
    // how far plus and minus to go surrounding SERVICE_UUID
    const RANGE: u128 = 32;

    const COMMON_PART: u128 = SERVICE_UUID.as_u128() & SERVICE_UUID_MASK.as_u128();

    // keep only the first two bytes of the device specific part (0xXX000000-...)
    const DEVICE_SPECIFIC_PART_SHIFT: u32 = u128::BITS - 16;

    const DEVICE_SPECIFIC_PART_CENTER: u128 = SERVICE_UUID.as_u128() >> DEVICE_SPECIFIC_PART_SHIFT;
    const MIN: u128 = DEVICE_SPECIFIC_PART_CENTER.wrapping_sub(RANGE);
    const MAX: u128 = DEVICE_SPECIFIC_PART_CENTER + RANGE;
    const_assert!(MIN < MAX);
    const_assert!(MAX <= 0xFFFF); // should not overflow

    (MIN..=MAX)
        .map(|device_specific_part| {
            let uuid = (device_specific_part << DEVICE_SPECIFIC_PART_SHIFT) | COMMON_PART;
            Uuid::from_u128(uuid)
        })
        .collect::<Vec<_>>()
}

// All mac address prefixes owned by the following companies should be listed here.
// See: http://standards-oui.ieee.org/oui/oui.csv
const MAC_ADDRESS_PREFIXES: [[u8; 3]; 8] = [
    // Fantasia Trading LLC
    [0xAC, 0x12, 0x2F],
    [0xE8, 0xEE, 0xCC],
    [0xF4, 0x9D, 0x8A],
    // Ningbo FreeWings Technologies Co.,Ltd
    [0xA4, 0x77, 0x58],
    [0xA0, 0xE9, 0xDB],
    // Shenzhen Boomtech Industrial Corporation
    [0x98, 0x47, 0x44],
    // ???
    [0xE4, 0x9E, 0x58],
    [0x88, 0x0E, 0x85],
];
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
    use super::*;
    use std::str::FromStr;
    use uuid::{Uuid, uuid};

    #[test]
    fn test_center_service_uuid() {
        assert!(is_soundcore_service_uuid(&SERVICE_UUID));
    }

    #[test]
    fn test_valid_service_uuid() {
        let uuid = Uuid::from_str("1234f5da-0000-1000-8000-00805f9b34fb").unwrap();
        assert!(is_soundcore_service_uuid(&uuid));
    }

    #[test]
    fn test_invalid_service_uuid() {
        let uuid = Uuid::from_str("123455da-0000-1000-8000-00805f9b34fb").unwrap();
        assert!(!is_soundcore_service_uuid(&uuid));
    }

    #[test]
    fn test_service_uuids_are_in_correct_range() {
        let uuids = service_uuids();
        assert!(uuids.contains(&uuid!("0100f5da-0000-1000-8000-00805f9b34fb")));
        assert!(uuids.contains(&uuid!("011cf5da-0000-1000-8000-00805f9b34fb")));
        assert!(uuids.contains(&uuid!("0120f5da-0000-1000-8000-00805f9b34fb")));
    }

    #[test]
    fn test_soundcore_device_mac_address() {
        let mac_address = [0xAC, 0x12, 0x2F, 0x00, 0x00, 0x00].into();
        assert!(is_mac_address_soundcore_device(mac_address));
    }

    #[test]
    fn test_not_soundcore_device_mac_address() {
        let mac_address = [0xAC, 0x00, 0x00, 0x00, 0x00, 0x00].into();
        assert!(!is_mac_address_soundcore_device(mac_address));
    }
}

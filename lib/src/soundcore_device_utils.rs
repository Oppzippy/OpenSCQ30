use uuid::Uuid;

pub const SERVICE_UUID: Uuid = uuid::uuid!("011cf5da-0000-1000-8000-00805f9b34fb");
pub const WRITE_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("00007777-0000-1000-8000-00805f9b34fb");
pub const READ_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("00008888-0000-1000-8000-00805F9B34FB");

pub fn is_mac_address_soundcore_device(mac_address: [u8; 6]) -> bool {
    mac_address.starts_with(&[0xAC, 0x12, 0x2F])
}

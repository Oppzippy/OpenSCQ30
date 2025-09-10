pub mod a3004;
pub mod a3027;
pub mod a3028;
pub mod a3031;
pub mod a3033;
pub mod a3926;
pub mod a3930;
pub mod a3931;
pub mod a3933;
pub mod a3936;
pub mod a3945;
pub mod a3948;
pub mod a3951;
pub mod a3959;
pub mod common;
pub mod development;

use uuid::{Uuid, uuid};

const RFCOMM_UUID: Uuid = uuid!("00001101-0000-1000-8000-00805f9b34fb");
const VENDOR_RFCOMM_UUID: Uuid = uuid!("0cf12d31-fac3-4553-bd80-d6832e7b0000");
const VENDOR_RFCOMM_UUID_MASK: Uuid = uuid!("ffffffff-ffff-ffff-ffff-ffffffff0000");

fn is_soundcore_vendor_rfcomm_uuid(uuid: &Uuid) -> bool {
    let mask = VENDOR_RFCOMM_UUID_MASK.as_u128();
    uuid.as_u128() & mask == VENDOR_RFCOMM_UUID.as_u128() & mask
}

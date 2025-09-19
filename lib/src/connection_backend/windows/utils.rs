use std::panic::Location;

use macaddr::MacAddr6;
use uuid::Uuid;
use windows::core::GUID;

use crate::api::connection;

pub trait WindowsMacAddressExt {
    #[cfg(test)]
    fn as_windows_u64(&self) -> u64;
    fn from_windows_u64(value: u64) -> Self;
}

impl WindowsMacAddressExt for MacAddr6 {
    #[cfg(test)]
    fn as_windows_u64(&self) -> u64 {
        let bytes = self.into_array();
        let length = bytes.len();
        bytes
            .into_iter()
            .enumerate()
            .fold(0_u64, |acc, (index, value)| {
                let inverse_index = (length - 1) - index;
                acc | ((value as u64) << (inverse_index * 8))
            })
    }

    fn from_windows_u64(value: u64) -> Self {
        let address_bytes: [u8; 6] = value.to_be_bytes()[2..8]
            .try_into()
            .expect("expected 6 byte mac address");
        Self::from(address_bytes)
    }
}

pub trait GuidAsUuidExt {
    fn as_uuid(&self) -> Uuid;
}

impl GuidAsUuidExt for GUID {
    fn as_uuid(&self) -> Uuid {
        Uuid::from_u128(self.to_u128())
    }
}

impl From<windows::core::Error> for connection::Error {
    #[track_caller]
    fn from(err: windows::core::Error) -> Self {
        match err.code().0 as u32 {
            0x80000013 => Self::DeviceNotFound {
                source: Some(Box::new(err)),
                location: Location::caller(),
            },
            _ => Self::Other {
                source: Box::new(err),
                location: Location::caller(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WindowsMacAddressExt;
    use macaddr::MacAddr6;
    use std::str::FromStr;

    #[test]
    fn test_original_equals_to_and_from() {
        let mac_address = MacAddr6::from_str("01:23:45:67:89:AB").unwrap();
        let windows_u64 = mac_address.as_windows_u64();
        let from_windows_u64 = MacAddr6::from_windows_u64(windows_u64);
        assert_eq!(mac_address, from_windows_u64);
    }

    #[test]
    fn test_to() {
        let mac_address = MacAddr6::from_str("01:23:45:67:89:AB").unwrap();
        assert_eq!(mac_address.as_windows_u64(), 0x0123456789AB);
    }

    #[test]
    fn test_from() {
        let mac_address = MacAddr6::from_windows_u64(0x0123456789AB);
        assert_eq!(
            mac_address,
            MacAddr6::from_str("01:23:45:67:89:AB").unwrap()
        );
    }
}

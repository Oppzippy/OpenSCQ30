use macaddr::MacAddr6;

pub trait WindowsMacAddress {
    fn as_windows_u64(&self) -> u64;
    fn from_windows_u64(value: u64) -> Self;
}

impl WindowsMacAddress for MacAddr6 {
    fn as_windows_u64(&self) -> u64 {
        self.into_array()
            .into_iter()
            .enumerate()
            .fold(0 as u64, |acc, (i, value)| {
                acc | ((value as u64) << (i) * 8)
            })
    }

    fn from_windows_u64(value: u64) -> Self {
        let address_bytes: [u8; 6] = value.to_le_bytes()[0..6]
            .try_into()
            .expect("expected 6 byte mac address");
        Self::from(address_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::WindowsMacAddress;
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
        // The bytes should be flipped around
        assert_eq!(mac_address.as_windows_u64(), 0xAB8967452301);
    }

    #[test]
    fn test_from() {
        let mac_address = MacAddr6::from_windows_u64(0xAB8967452301);
        // The bytes should be flipped around
        assert_eq!(
            mac_address,
            MacAddr6::from_str("01:23:45:67:89:AB").unwrap()
        );
    }
}

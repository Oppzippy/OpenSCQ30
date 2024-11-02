#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, VariantArray};

use crate::devices::standard::structures::SerialNumber;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, VariantArray, AsRefStr, Display)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum DeviceModel {
    A3027,
    A3028,
    A3029,
    A3030,
    A3031,
    A3033,
    A3926,
    A3930,
    A3931,
    A3933,
    A3936,
    A3945,
    A3951,
    A3939,
    A3935,
}

impl DeviceModel {
    pub fn from_serial_number(serial_number: &SerialNumber) -> Option<Self> {
        Self::from_str(&serial_number.as_str()[12..])
    }

    fn from_str(model_number: &str) -> Option<Self> {
        Self::VARIANTS
            .iter()
            .find(|model| &model.as_ref()[1..] == model_number)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_serial_number() {
        assert!(DeviceModel::from_serial_number(&"0000000000003028".into()).is_some());
    }

    #[test]
    fn test_invalid_serial_number() {
        assert!(DeviceModel::from_serial_number(&"0000000000000000".into()).is_none());
    }
}

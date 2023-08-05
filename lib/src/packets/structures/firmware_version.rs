use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct FirmwareVersion(pub String);

impl From<FirmwareVersion> for String {
    fn from(value: FirmwareVersion) -> Self {
        value.0
    }
}

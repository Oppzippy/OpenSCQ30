use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct SerialNumber(pub String);

impl From<SerialNumber> for String {
    fn from(value: SerialNumber) -> Self {
        value.0
    }
}

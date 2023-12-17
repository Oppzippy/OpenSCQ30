use std::sync::Arc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SerialNumber(pub Arc<str>);

impl SerialNumber {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SerialNumber {
    fn default() -> Self {
        Self("0000000000000000".into())
    }
}

impl From<&str> for SerialNumber {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl ToString for SerialNumber {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

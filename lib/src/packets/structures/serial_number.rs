use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SerialNumber(pub Arc<str>);

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

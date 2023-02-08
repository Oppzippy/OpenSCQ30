use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EqualizerCustomProfile {
    pub name: String,
    pub volume_offsets: [u8; 8],
}

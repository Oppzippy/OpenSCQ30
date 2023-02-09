use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EqualizerCustomProfile {
    pub volume_offsets: [i8; 8],
}

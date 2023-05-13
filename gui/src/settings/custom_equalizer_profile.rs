use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomEqualizerProfile {
    // Not renamed to volume_adjustments to keep backwards compatibility with old settings files
    volume_offsets: [i8; 8],
}

impl CustomEqualizerProfile {
    pub fn new(volume_adjustments: [i8; 8]) -> Self {
        Self {
            volume_offsets: volume_adjustments,
        }
    }

    pub fn volume_adjustments(&self) -> [i8; 8] {
        self.volume_offsets
    }
}

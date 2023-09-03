use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomEqualizerProfile {
    // Not renamed to volume_adjustments to keep backwards compatibility with old settings files
    volume_offsets: [i16; 8],
}

impl CustomEqualizerProfile {
    pub fn new(volume_adjustments: [f64; 8]) -> Self {
        // Convert f64 to i16 for backwards compatibility with serialization format
        Self {
            volume_offsets: volume_adjustments.map(|value| (value * 10.0).round() as i16),
        }
    }

    pub fn volume_adjustments(&self) -> [f64; 8] {
        self.volume_offsets
            .map(|adjustment| (adjustment as f64) / 10.0)
    }
}

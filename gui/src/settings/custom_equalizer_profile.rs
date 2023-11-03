use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomEqualizerProfile {
    // Not renamed to volume_adjustments to keep backwards compatibility with old settings files
    volume_offsets: Vec<i16>,
}

impl CustomEqualizerProfile {
    pub fn new(volume_adjustments: &[f64]) -> Self {
        // Convert f64 to i16 for backwards compatibility with serialization format
        Self {
            volume_offsets: volume_adjustments
                .iter()
                .map(|value| (value * 10.0).round() as i16)
                .collect(),
        }
    }

    pub fn volume_adjustments(&self) -> Arc<[f64]> {
        self.volume_offsets
            .iter()
            .map(|adjustment| (*adjustment as f64) / 10.0)
            .collect()
    }
}

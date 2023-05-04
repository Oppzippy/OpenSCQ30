use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomEqualizerProfile {
    volume_offsets: [i8; 8],
}

impl CustomEqualizerProfile {
    pub fn new(volume_offsets: [i8; 8]) -> Self {
        Self { volume_offsets }
    }

    pub fn volume_offsets(&self) -> [i8; 8] {
        self.volume_offsets
    }
}

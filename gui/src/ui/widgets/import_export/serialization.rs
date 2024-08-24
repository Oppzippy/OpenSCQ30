use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::objects::GlibCustomEqualizerProfile;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IOCustomEqualizerProfile {
    pub name: String,
    pub volume_adjustments: Arc<[f64]>,
}

impl From<GlibCustomEqualizerProfile> for IOCustomEqualizerProfile {
    fn from(value: GlibCustomEqualizerProfile) -> Self {
        Self {
            name: value.name(),
            volume_adjustments: value.volume_adjustments(),
        }
    }
}

impl From<IOCustomEqualizerProfile> for GlibCustomEqualizerProfile {
    fn from(value: IOCustomEqualizerProfile) -> Self {
        Self::new(&value.name, value.volume_adjustments)
    }
}

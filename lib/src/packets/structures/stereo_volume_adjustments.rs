use serde::{Deserialize, Serialize};

use super::VolumeAdjustments;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StereoVolumeAdjustments {
    pub left: VolumeAdjustments,
    pub right: VolumeAdjustments,
}

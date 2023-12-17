#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::StereoVolumeAdjustments;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BasicHearId {
    pub is_enabled: bool,
    pub volume_adjustments: StereoVolumeAdjustments,
    pub time: i32,
}

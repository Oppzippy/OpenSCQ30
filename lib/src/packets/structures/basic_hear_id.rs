use super::StereoVolumeAdjustments;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BasicHearId {
    pub is_enabled: bool,
    pub volume_adjustments: StereoVolumeAdjustments,
    pub time: i32,
}

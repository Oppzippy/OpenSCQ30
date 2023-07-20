use super::VolumeAdjustments;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HearId {
    pub is_enabled: bool,
    pub left: VolumeAdjustments,
    pub right: VolumeAdjustments,
    pub time: i32,
}

use super::VolumeAdjustments;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct StereoVolumeAdjustments {
    pub left: VolumeAdjustments,
    pub right: VolumeAdjustments,
}

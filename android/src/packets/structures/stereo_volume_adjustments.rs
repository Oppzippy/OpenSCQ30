use super::VolumeAdjustments;
use openscq30_lib::packets::structures::StereoVolumeAdjustments as LibStereoVolumeAdjustments;
use rifgen::rifgen_attr::generate_interface;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct StereoVolumeAdjustments(LibStereoVolumeAdjustments);

impl StereoVolumeAdjustments {
    #[generate_interface(constructor)]
    pub fn new(left: VolumeAdjustments, right: VolumeAdjustments) -> StereoVolumeAdjustments {
        Self(LibStereoVolumeAdjustments {
            left: left.into(),
            right: right.into(),
        })
    }

    #[generate_interface]
    pub fn left(&self) -> VolumeAdjustments {
        self.0.left.into()
    }

    #[generate_interface]
    pub fn right(&self) -> VolumeAdjustments {
        self.0.right.into()
    }
}

impl From<LibStereoVolumeAdjustments> for StereoVolumeAdjustments {
    fn from(value: LibStereoVolumeAdjustments) -> Self {
        Self(value)
    }
}

impl From<StereoVolumeAdjustments> for LibStereoVolumeAdjustments {
    fn from(value: StereoVolumeAdjustments) -> Self {
        value.0
    }
}
use super::VolumeAdjustments;
use openscq30_lib::packets::structures::StereoVolumeAdjustments as LibStereoVolumeAdjustments;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct StereoVolumeAdjustments {
    pub left: VolumeAdjustments,
    pub right: VolumeAdjustments,
}

impl From<LibStereoVolumeAdjustments> for StereoVolumeAdjustments {
    fn from(value: LibStereoVolumeAdjustments) -> Self {
        Self {
            left: value.left.into(),
            right: value.right.into(),
        }
    }
}

impl From<StereoVolumeAdjustments> for LibStereoVolumeAdjustments {
    fn from(value: StereoVolumeAdjustments) -> Self {
        Self {
            left: value.left.into(),
            right: value.right.into(),
        }
    }
}

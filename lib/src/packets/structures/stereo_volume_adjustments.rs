use std::array;

use serde::{Deserialize, Serialize};

use super::VolumeAdjustments;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StereoVolumeAdjustments {
    pub left: VolumeAdjustments,
    pub right: VolumeAdjustments,
}

impl StereoVolumeAdjustments {
    pub fn bytes(&self) -> [u8; 16] {
        let left_bytes = self.left.bytes();
        let right_bytes = self.right.bytes();
        array::from_fn(|i| {
            if i < 8 {
                left_bytes[i]
            } else {
                right_bytes[i - 8]
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::structures::VolumeAdjustments;

    use super::StereoVolumeAdjustments;

    #[test]
    fn it_orders_bytes_correctly() {
        let stereo_volume_adjustments = StereoVolumeAdjustments {
            left: VolumeAdjustments::new([0, 1, 2, 3, 4, 5, 6, 7]),
            right: VolumeAdjustments::new([8, 9, 10, 11, 12, 13, 14, 15]),
        };
        let bytes = stereo_volume_adjustments.bytes();
        assert_eq!(stereo_volume_adjustments.left.bytes(), bytes[0..8]);
        assert_eq!(stereo_volume_adjustments.right.bytes(), bytes[8..16]);
    }
}

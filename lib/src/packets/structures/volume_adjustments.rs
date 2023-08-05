use std::array;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VolumeAdjustments {
    volume_adjustments: [i8; 8],
}

impl VolumeAdjustments {
    pub const MIN_VOLUME: i8 = -120;
    pub const MAX_VOLUME: i8 = 120;

    pub fn new(volume_adjustments: [i8; 8]) -> Self {
        let clamped_adjustments =
            volume_adjustments.map(|vol| vol.clamp(Self::MIN_VOLUME, Self::MAX_VOLUME));
        Self {
            volume_adjustments: clamped_adjustments,
        }
    }

    pub fn bytes(&self) -> [u8; 8] {
        self.volume_adjustments
            .map(Self::signed_adjustment_to_packet_byte)
    }

    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        Self::new(bytes.map(Self::packet_byte_to_signed_adjustment))
    }

    pub fn adjustments(&self) -> [i8; 8] {
        self.volume_adjustments
    }

    fn signed_adjustment_to_packet_byte(offset: i8) -> u8 {
        let clamped = offset.clamp(Self::MIN_VOLUME, Self::MAX_VOLUME);
        clamped.wrapping_add(Self::MIN_VOLUME.abs()) as u8
    }

    fn packet_byte_to_signed_adjustment(byte: u8) -> i8 {
        let clamped = byte.clamp(
            Self::signed_adjustment_to_packet_byte(Self::MIN_VOLUME),
            Self::signed_adjustment_to_packet_byte(Self::MAX_VOLUME),
        );
        clamped.wrapping_sub(Self::MIN_VOLUME.unsigned_abs()) as i8
    }
}

// need drc:                     A3951, A3930, A3931, A3931XR, A3935, A3935W,
// separate left/right firmware: A3951, A3930, A3931, A3931XR, A3935, A3935W,
pub fn apply_drc(volume_adjustments: VolumeAdjustments) -> VolumeAdjustments {
    let pre_drc_subtraction = 12.0;
    let bands = volume_adjustments
        .adjustments()
        .map(|x| f64::from(x) * 10.0)
        .map(|x| x - pre_drc_subtraction);
    let smaller_coefficient = 0.85;
    let larger_coefficient = 0.95;
    let lows_coefficient = bands[1] * 0.81 * smaller_coefficient;
    let mids_coefficient = bands[4] * 0.81 * smaller_coefficient;

    let band_coefficients = [
        // 0
        [
            1.26,
            -0.71 * smaller_coefficient,
            0.177,
            -0.0494,
            0.0345,
            -0.0197,
            0.0075,
            -0.00217,
        ],
        // 1
        [
            0.0345,
            -0.068,
            0.208,
            -0.82 * smaller_coefficient,
            1.73 * larger_coefficient,
            -mids_coefficient,
            0.204,
            -0.0494,
        ],
        // 2
        [
            -0.00217,
            0.0075,
            -0.0197,
            0.0345,
            -0.0494,
            0.177,
            -0.71 * smaller_coefficient,
            1.5,
        ],
        // 3
        [
            -0.0494,
            0.204,
            -lows_coefficient,
            1.73 * larger_coefficient,
            -0.82 * smaller_coefficient,
            0.208,
            -0.068,
            0.0345,
        ],
        // 4
        [
            0.0345,
            -0.068,
            0.208,
            -0.82 * smaller_coefficient,
            1.73 * larger_coefficient,
            -mids_coefficient,
            0.204,
            -0.0494,
        ],
        // 5
        [
            -0.0197,
            0.045,
            -0.07,
            0.208,
            -0.81 * smaller_coefficient,
            1.73 * larger_coefficient,
            -0.81 * smaller_coefficient,
            0.177,
        ],
        // 6
        [
            0.0075,
            -0.0235,
            0.045,
            -0.068,
            0.204,
            -mids_coefficient,
            1.83 * larger_coefficient,
            -0.71 * smaller_coefficient,
        ],
        // 7
        [
            -0.00217,
            0.0075,
            -0.0197,
            0.0345,
            -0.0494,
            0.177,
            -0.71 * smaller_coefficient,
            1.5,
        ],
    ];

    let multiplied_bands: [f64; 8] = array::from_fn(|index| {
        let coefficients = band_coefficients[index];
        bands
            .iter()
            .enumerate()
            .fold(0.0, |acc, (index, curr)| acc + curr * coefficients[index])
    });

    let byte_bands = multiplied_bands
        .map(|band| band + pre_drc_subtraction)
        .map(|band| (band / 10.0) as i8);
    VolumeAdjustments::new(byte_bands)
}

impl From<VolumeAdjustments> for [i8; 8] {
    fn from(volume_adjustments: VolumeAdjustments) -> Self {
        volume_adjustments.adjustments()
    }
}

#[cfg(test)]
mod tests {
    use super::VolumeAdjustments;
    const TEST_BYTES: [u8; 8] = [0, 80, 100, 120, 140, 160, 180, 240];
    const TEST_ADJUSTMENTS: [i8; 8] = [-120, -40, -20, 0, 20, 40, 60, 120];

    #[test]
    fn converts_volume_adjustments_to_packet_bytes() {
        let band_adjustments = VolumeAdjustments::new(TEST_ADJUSTMENTS);
        assert_eq!(TEST_BYTES, band_adjustments.bytes());
    }

    #[test]
    fn from_bytes_converts_packet_bytes_to_adjustment() {
        let band_adjustments = VolumeAdjustments::from_bytes(TEST_BYTES);
        assert_eq!(TEST_ADJUSTMENTS, band_adjustments.adjustments());
    }

    #[test]
    fn it_clamps_bytes_outside_of_expected_range() {
        let band_adjustments =
            VolumeAdjustments::from_bytes([0, 255, 120, 120, 120, 120, 120, 120]);
        assert_eq!(
            [0, 240, 120, 120, 120, 120, 120, 120],
            band_adjustments.bytes()
        );
    }

    #[test]
    fn it_clamps_volume_adjustments_outside_of_expected_range() {
        let band_adjustments = VolumeAdjustments::new([-128, 127, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            [-120, 120, 0, 0, 0, 0, 0, 0],
            band_adjustments.adjustments()
        );
    }
}

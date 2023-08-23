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

    pub fn apply_drc(&self) -> VolumeAdjustments {
        const PRE_DRC_SUBTRACTION: f64 = 12.0;
        let bands = self
            .adjustments()
            // go from -120 to -120 to -12.0 to 12.0
            .map(|x| f64::from(x) / 10.0)
            // a constant is subtracted before doing calculations, and added back at the end
            .map(|x| x - PRE_DRC_SUBTRACTION);

        const SMALLER_COEFFICIENT: f64 = 0.85;
        const LARGER_COEFFICIENT: f64 = 0.95;
        let lows_subtraction = bands[2] * 0.81 * SMALLER_COEFFICIENT;
        let highs_subtraction = bands[5] * 0.81 * SMALLER_COEFFICIENT;

        let band_coefficients = [
            // 0
            [
                1.26,
                -0.71 * SMALLER_COEFFICIENT,
                0.177,
                -0.0494,
                0.0345,
                -0.0197,
                0.0075,
                -0.00217,
            ],
            // 1
            [
                -0.71 * SMALLER_COEFFICIENT,
                1.73 * LARGER_COEFFICIENT,
                0.0, // subtraction
                0.204,
                -0.068,
                0.045,
                -0.0235,
                0.0075,
            ],
            // 2
            [
                0.177,
                -0.81 * SMALLER_COEFFICIENT,
                1.73 * LARGER_COEFFICIENT,
                -0.81 * SMALLER_COEFFICIENT,
                0.208,
                -0.07,
                0.045,
                -0.0197,
            ],
            // 3
            [
                -0.0494,
                0.204,
                0.0, // subtraction
                1.73 * LARGER_COEFFICIENT,
                -0.82 * SMALLER_COEFFICIENT,
                0.208,
                -0.068,
                0.0345,
            ],
            // 4
            [
                0.0345,
                -0.068,
                0.208,
                -0.82 * SMALLER_COEFFICIENT,
                1.73 * LARGER_COEFFICIENT,
                0.0, // subtraction
                0.204,
                -0.0494,
            ],
            // 5
            [
                -0.0197,
                0.045,
                -0.07,
                0.208,
                -0.81 * SMALLER_COEFFICIENT,
                1.73 * LARGER_COEFFICIENT,
                -0.81 * SMALLER_COEFFICIENT,
                0.177,
            ],
            // 6
            [
                0.0075,
                -0.0235,
                0.045,
                -0.068,
                0.204,
                0.0, // subtraction
                1.83 * LARGER_COEFFICIENT,
                -0.71 * SMALLER_COEFFICIENT,
            ],
            // 7
            [
                -0.00217,
                0.0075,
                -0.0197,
                0.0345,
                -0.0494,
                0.177,
                -0.71 * SMALLER_COEFFICIENT,
                1.5,
            ],
        ];

        // if we were to do band_coefficients.map, we wouldn't get the index, and if we converted
        // to an iterator and used enumerate, we can't easily collect back into an array.
        let multiplied_bands: [f64; 8] = array::from_fn(|band_coefficient_index| {
            let coefficients = band_coefficients[band_coefficient_index];
            bands.iter().enumerate().fold(0.0, |acc, (index, curr)| {
                if (band_coefficient_index == 1 || band_coefficient_index == 3) && index == 2 {
                    acc - lows_subtraction
                } else if (band_coefficient_index == 4 || band_coefficient_index == 6) && index == 5
                {
                    acc - highs_subtraction
                } else {
                    acc + curr * coefficients[index]
                }
            })
        });

        let byte_bands = multiplied_bands
            .map(|band| band + PRE_DRC_SUBTRACTION * 10.0)
            // scale back up to -120 to 120
            .map(|band| band.round().clamp(i8::MIN as f64, i8::MAX as f64) as i8);
        VolumeAdjustments::new(byte_bands)
    }
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

    #[test]
    fn it_matches_expected_drc_values() {
        let examples = [
            (
                [-60, 60, 23, 120, 22, -120, -4, 16], // volume adjustments
                [99, 127, 104, 127 /* 129 */, 116, 95, 119, 108], // drc
            ),
            (
                [120, 120, 120, 120, 120, 120, 120, 120],
                [120, 120, 120, 120, 120, 120, 120, 120],
            ),
            (
                [-120, -120, -120, -120, -120, -120, -120, -120],
                [101, 108, 105, 106, 106, 105, 105, 95],
            ),
            (
                [-60, 60, 23, 120, 22, -120, -4, 16],
                [99, 127, 104, 127, 116, 95, 119, 108],
            ),
        ];

        for example in examples {
            let actual = VolumeAdjustments::new(example.0).apply_drc();
            let expected = VolumeAdjustments::new(example.1);
            assert_eq!(expected, actual);
        }
    }
}

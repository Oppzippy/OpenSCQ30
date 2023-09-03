use std::array;

use float_cmp::{ApproxEq, F64Margin};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct VolumeAdjustments {
    volume_adjustments: [OrderedFloat<f64>; 8],
}

impl VolumeAdjustments {
    pub const STEP: f64 = 0.1;
    pub const MIN_VOLUME: f64 = -12.0;
    pub const MAX_VOLUME: f64 = ((u8::MAX - 120) as f64) / 10.0;
    pub const MARGIN: F64Margin = F64Margin {
        epsilon: f64::EPSILON * 20.0,
        ulps: 4,
    };

    pub fn new(volume_adjustments: [f64; 8]) -> Self {
        let clamped_adjustments =
            volume_adjustments.map(|vol| vol.clamp(Self::MIN_VOLUME, Self::MAX_VOLUME));
        Self {
            volume_adjustments: clamped_adjustments.map(Into::into),
        }
    }

    pub fn bytes(&self) -> [u8; 8] {
        self.volume_adjustments
            .map(|adjustment| Self::signed_adjustment_to_packet_byte(adjustment.into()))
    }

    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        Self::new(bytes.map(Self::packet_byte_to_signed_adjustment))
    }

    pub fn adjustments(&self) -> [f64; 8] {
        self.volume_adjustments.map(|adjustment| adjustment.into())
    }

    fn signed_adjustment_to_packet_byte(adjustment: f64) -> u8 {
        let clamped = adjustment.clamp(Self::MIN_VOLUME, Self::MAX_VOLUME);
        let shifted = (clamped - Self::MIN_VOLUME) * 10.0;
        shifted.round() as u8
    }

    fn packet_byte_to_signed_adjustment(byte: u8) -> f64 {
        (byte as f64) / 10.0 + Self::MIN_VOLUME
    }

    pub fn apply_drc(&self) -> VolumeAdjustments {
        const PRE_DRC_SUBTRACTION: f64 = 12.0;
        let bands = self
            .adjustments()
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

        let byte_bands = multiplied_bands.map(|band| (band / 10.0) + PRE_DRC_SUBTRACTION);
        VolumeAdjustments::new(byte_bands)
    }
}

impl ApproxEq for VolumeAdjustments {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.adjustments()
            .into_iter()
            .zip(other.adjustments())
            .all(|(left, right)| left.approx_eq(right, margin))
    }
}

impl From<VolumeAdjustments> for [f64; 8] {
    fn from(volume_adjustments: VolumeAdjustments) -> Self {
        volume_adjustments.adjustments()
    }
}

impl From<[f64; 8]> for VolumeAdjustments {
    fn from(value: [f64; 8]) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::{assert_approx_eq, F64Margin};

    use super::VolumeAdjustments;
    const TEST_BYTES: [u8; 8] = [0, 80, 100, 120, 140, 160, 180, 240];
    const TEST_ADJUSTMENTS: [f64; 8] = [-12.0, -4.0, -2.0, 0.0, 2.0, 4.0, 6.0, 12.0];

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
    fn it_clamps_volume_adjustments_outside_of_expected_range() {
        let band_adjustments =
            VolumeAdjustments::new([f64::MIN, f64::MAX, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(
            [
                VolumeAdjustments::MIN_VOLUME,
                VolumeAdjustments::MAX_VOLUME,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            ],
            band_adjustments.adjustments()
        );
    }

    #[test]
    fn it_matches_expected_drc_values() {
        let examples = [
            (
                [-6.0, 6.0, 2.3, 12.0, 2.2, -12.0, -0.4, 1.6], // volume adjustments
                [
                    9.928837, 12.748025, 10.429153, 12.866665, 11.617126, 9.47635, 11.8813305,
                    10.830467,
                ], // drc
            ),
            (
                [12.0, 12.0, 12.0, 12.0, 12.0, 12.0, 12.0, 12.0],
                [12.0, 12.0, 12.0, 12.0, 12.0, 12.0, 12.0, 12.0],
            ),
            (
                [-12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0],
                [
                    10.069848, 10.7604, 10.54368, 10.59096, 10.59096, 10.54368, 10.5324, 9.493848,
                ],
            ),
            (
                [-6.0, 6.0, 2.3, 12.0, 2.2, -12.0, -0.4, 1.6],
                [
                    9.928837, 12.748025, 10.429153, 12.866665, 11.617126, 9.47635, 11.8813305,
                    10.830467,
                ],
            ),
        ];

        for example in examples {
            let actual = VolumeAdjustments::new(example.0).apply_drc();
            let expected = VolumeAdjustments::new(example.1);
            assert_approx_eq!(
                VolumeAdjustments,
                expected,
                actual,
                F64Margin {
                    epsilon: f32::EPSILON as f64 * 20.0,
                    ..Default::default()
                }
            );
        }
    }
}

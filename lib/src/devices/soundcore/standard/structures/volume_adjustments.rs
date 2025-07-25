use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};
use std::array;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VolumeAdjustments<const BANDS: usize> {
    inner: [i16; BANDS],
}

impl<const B: usize> Default for VolumeAdjustments<B> {
    fn default() -> Self {
        Self { inner: [0; B] }
    }
}

const FRACTION_DIGITS: u8 = 1;
const MIN_VOLUME: i16 = -120;
const MAX_VOLUME: i16 = (u8::MAX - 121) as i16;

impl<const B: usize> VolumeAdjustments<B> {
    pub fn new(volume_adjustments: [i16; B]) -> Self {
        let clamped = volume_adjustments.map(|vol| vol.clamp(MIN_VOLUME, MAX_VOLUME));
        Self { inner: clamped }
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "volume adjustment",
            map(take(B), |volume_adjustment_bytes: &[u8]| {
                Self::from_bytes(
                    volume_adjustment_bytes
                        .try_into()
                        .expect("take guarantees that the length will be B"),
                )
            }),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; B] {
        self.inner.map(Self::signed_adjustment_to_packet_byte)
    }

    pub fn from_bytes(bytes: [u8; B]) -> Self {
        Self::new(bytes.map(Self::packet_byte_to_signed_adjustment))
    }

    pub fn adjustments(&self) -> &[i16; B] {
        &self.inner
    }

    fn signed_adjustment_to_packet_byte(adjustment: i16) -> u8 {
        let clamped = adjustment.clamp(MIN_VOLUME, MAX_VOLUME);
        let shifted = clamped - MIN_VOLUME;
        shifted
            .try_into()
            .expect("value is already clamped, so it can't overflow")
    }

    fn packet_byte_to_signed_adjustment(byte: u8) -> i16 {
        (byte as i16) + MIN_VOLUME
    }

    pub fn apply_drc(&self) -> Self {
        let adjustments = self
            .inner
            .map(|adjustment| adjustment as f64 / 10_i16.pow(FRACTION_DIGITS as u32) as f64);

        const SMALLER_COEFFICIENT: f64 = 0.85;
        const LARGER_COEFFICIENT: f64 = 0.95;
        let lows_subtraction = adjustments[2] * 0.81 * SMALLER_COEFFICIENT;
        let highs_subtraction = adjustments[5] * 0.81 * SMALLER_COEFFICIENT;

        const BAND_COEFFICIENTS: [[f64; 8]; 8] = [
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

        // The DRC algorithm only affects the first 8 bands
        // if we were to do band_coefficients.map, we wouldn't get the index, and if we converted
        // to an iterator and used enumerate, we can't easily collect back into an array.
        let multiplied_adjustments_8_bands: [f64; 8] = array::from_fn(|band_coefficient_index| {
            let coefficients = BAND_COEFFICIENTS[band_coefficient_index];
            adjustments
                .iter()
                .take(8)
                .enumerate()
                .fold(0.0, |acc, (index, adjustment)| {
                    // Some special cases where a particular band's adjustment is not factored in
                    if (band_coefficient_index == 1 || band_coefficient_index == 3) && index == 2 {
                        acc - lows_subtraction
                    } else if (band_coefficient_index == 4 || band_coefficient_index == 6)
                        && index == 5
                    {
                        acc - highs_subtraction
                    } else {
                        acc + adjustment * coefficients[index]
                    }
                })
        });
        // divide afterwards
        let new_adjustments_8_bands = multiplied_adjustments_8_bands.map(|band| band / 10.0);

        // Add bands 9+ back on to the end
        let new_adjustments_with_all_bands: [i16; B] = array::from_fn(|i| {
            new_adjustments_8_bands
                .get(i)
                .copied()
                .unwrap_or(adjustments[i])
        })
        .map(|adjustment| (adjustment * 10_i16.pow(FRACTION_DIGITS as u32) as f64).round() as i16);

        Self::new(new_adjustments_with_all_bands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_BYTES: [u8; 8] = [0, 80, 100, 120, 140, 160, 180, 240];
    const TEST_ADJUSTMENTS: [i16; 8] = [-120, -40, -20, 0, 20, 40, 60, 120];

    #[test]
    fn converts_volume_adjustments_to_packet_bytes() {
        let band_adjustments = VolumeAdjustments::new(TEST_ADJUSTMENTS);
        assert_eq!(TEST_BYTES, band_adjustments.bytes());
    }

    #[test]
    fn from_bytes_converts_packet_bytes_to_adjustment() {
        let band_adjustments = VolumeAdjustments::from_bytes(TEST_BYTES);
        assert_eq!(TEST_ADJUSTMENTS, band_adjustments.adjustments().as_ref());
    }

    #[test]
    fn it_clamps_volume_adjustments_outside_of_expected_range() {
        let band_adjustments = VolumeAdjustments::new([i16::MIN, i16::MAX, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            [MIN_VOLUME, MAX_VOLUME, 0, 0, 0, 0, 0, 0,],
            band_adjustments.adjustments().as_ref()
        );
    }

    #[test]
    fn it_matches_expected_drc_values() {
        let examples = [
            (
                [-60, 60, 23, 120, 22, -120, -04, 16], // volume adjustments
                [
                    -1.1060872, 1.367825, -0.842687, 1.571185, 0.321646, -1.79549, 0.61513,
                    0.083543,
                ], // drc
            ),
            (
                [120, 120, 120, 120, 120, 120, 120, 120],
                [
                    0.96507597, 0.6198, 0.72816, 0.70452, 0.70452, 0.72816, 0.7338, 1.253076,
                ],
            ),
            (
                [-120, -120, -120, -120, -120, -120, -120, -120],
                [
                    -0.96507597,
                    -0.6198,
                    -0.72816,
                    -0.70452,
                    -0.70452,
                    -0.72816,
                    -0.7338,
                    -1.253076,
                ],
            ),
            (
                [-60, 60, 23, 120, 22, -120, -04, 16],
                [
                    -1.1060872, 1.367825, -0.842687, 1.571185, 0.321646, -1.79549, 0.61513,
                    0.083543,
                ],
            ),
            (
                [00, 00, 00, 00, 00, 00, 00, 00],
                [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            ),
        ];

        for example in examples {
            let actual = VolumeAdjustments::new(example.0).apply_drc();
            let expected =
                VolumeAdjustments::new(example.1.map(|v: f64| (v * 10f64).round() as i16));
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn it_does_not_modify_band_9_when_applying_drc() {
        let volume_adjustments = VolumeAdjustments::new([-60, 60, 23, 120, 22, -120, -04, 16, 50]);
        let expected = VolumeAdjustments::new(
            [
                -1.1060872, 1.367825, -0.842687, 1.571185, 0.321646, -1.79549, 0.61513, 0.083543,
                5.0,
            ]
            .map(|v| (v * 10.0f64).round() as i16),
        );
        let actual = volume_adjustments.apply_drc();
        assert_eq!(expected, actual);
    }
}

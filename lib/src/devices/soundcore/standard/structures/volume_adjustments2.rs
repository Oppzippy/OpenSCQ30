use nom::{
    IResult,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};
use std::{array, ops::RangeInclusive};

use float_cmp::F64Margin;

use super::VolumeAdjustments;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VolumeAdjustments2 {
    inner: Vec<i16>,
}

impl From<VolumeAdjustments> for VolumeAdjustments2 {
    fn from(value: VolumeAdjustments) -> Self {
        Self::new(
            value
                .adjustments()
                .iter()
                .map(|adjustment| (*adjustment * 10_f64).round() as i16)
                .collect(),
        )
        .unwrap()
    }
}

impl From<VolumeAdjustments2> for VolumeAdjustments {
    fn from(value: VolumeAdjustments2) -> Self {
        Self::new(
            value
                .adjustments()
                .iter()
                .map(|adjustment| *adjustment as f64 / 10_f64),
        )
        .unwrap()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum VolumeAdjustments2Error {
    #[error("invalid number of bands ({}), {values:?}", values.len())]
    InvalidNumberOfBands { values: Vec<i16> },
}

impl VolumeAdjustments2 {
    pub const FRACTION_DIGITS: u8 = 1;
    pub const MIN_VOLUME: i16 = -120;
    pub const MAX_VOLUME: i16 = (u8::MAX - 121) as i16;
    pub const MARGIN: F64Margin = F64Margin {
        epsilon: f32::EPSILON as f64 * 20.0,
        ulps: 4,
    };
    pub const VALID_NUMBER_OF_BANDS: RangeInclusive<usize> = 8..=10;

    pub fn new(volume_adjustments: Vec<i16>) -> Result<Self, VolumeAdjustments2Error> {
        let clamped = volume_adjustments
            .into_iter()
            .map(|vol| vol.clamp(Self::MIN_VOLUME, Self::MAX_VOLUME))
            .collect::<Vec<i16>>();
        if !Self::VALID_NUMBER_OF_BANDS.contains(&clamped.len()) {
            return Err(VolumeAdjustments2Error::InvalidNumberOfBands { values: clamped });
        }

        Ok(Self { inner: clamped })
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        num_bands: usize,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], VolumeAdjustments2, E> {
        move |input| {
            context(
                "volume adjustment",
                map(take(num_bands), |volume_adjustment_bytes: &[u8]| {
                    VolumeAdjustments2::from_bytes(volume_adjustment_bytes)
                        .expect("length was already verified by take(8)")
                }),
            )(input)
        }
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> + '_ {
        self.inner
            .iter()
            .cloned()
            .map(|adjustment| Self::signed_adjustment_to_packet_byte(adjustment.into()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VolumeAdjustments2Error> {
        Self::new(
            bytes
                .iter()
                .cloned()
                .map(Self::packet_byte_to_signed_adjustment)
                .collect(),
        )
    }

    pub fn adjustments(&self) -> &[i16] {
        &self.inner
    }

    fn signed_adjustment_to_packet_byte(adjustment: i16) -> u8 {
        let clamped = adjustment.clamp(Self::MIN_VOLUME, Self::MAX_VOLUME);
        let shifted = clamped - Self::MIN_VOLUME;
        shifted
            .try_into()
            .expect("value is already clamped, so it can't overflow")
    }

    fn packet_byte_to_signed_adjustment(byte: u8) -> i16 {
        (byte as i16) + (Self::MIN_VOLUME as i16)
    }

    pub fn apply_drc(&self) -> VolumeAdjustments2 {
        let adjustments = self
            .inner
            .iter()
            .map(|adjustment| *adjustment as f64 / 10_i16.pow(Self::FRACTION_DIGITS as u32) as f64)
            .collect::<Vec<_>>();

        const SMALLER_COEFFICIENT: f64 = 0.85;
        const LARGER_COEFFICIENT: f64 = 0.95;
        let lows_subtraction = adjustments[2] * 0.81 * SMALLER_COEFFICIENT;
        let highs_subtraction = adjustments[5] * 0.81 * SMALLER_COEFFICIENT;

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

        // The DRC algorithm only affects the first 8 bands
        // if we were to do band_coefficients.map, we wouldn't get the index, and if we converted
        // to an iterator and used enumerate, we can't easily collect back into an array.
        let multiplied_adjustments_8_bands: [f64; 8] = array::from_fn(|band_coefficient_index| {
            let coefficients = band_coefficients[band_coefficient_index];
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
        let new_adjustments_with_all_bands = new_adjustments_8_bands
            .into_iter()
            .chain(adjustments[8..].iter().cloned())
            // convert back to ints from floats
            .map(|adjustment| {
                (adjustment * 10_i16.pow(Self::FRACTION_DIGITS as u32) as f64).round() as i16
            })
            .collect::<Vec<_>>();

        VolumeAdjustments2::new(new_adjustments_with_all_bands).expect(
            "we are passing the same number of bands as self has, so it must be a valid number for self to exist",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::VolumeAdjustments2;
    const TEST_BYTES: [u8; 8] = [0, 80, 100, 120, 140, 160, 180, 240];
    const TEST_ADJUSTMENTS: [i16; 8] = [-120, -40, -20, 0, 20, 40, 60, 120];

    #[test]
    fn converts_volume_adjustments_to_packet_bytes() {
        let band_adjustments = VolumeAdjustments2::new(TEST_ADJUSTMENTS.to_vec()).unwrap();
        assert_eq!(
            TEST_BYTES,
            band_adjustments.bytes().collect::<Vec<_>>().as_ref()
        );
    }

    #[test]
    fn from_bytes_converts_packet_bytes_to_adjustment() {
        let band_adjustments = VolumeAdjustments2::from_bytes(&TEST_BYTES).unwrap();
        assert_eq!(TEST_ADJUSTMENTS, band_adjustments.adjustments().as_ref());
    }

    #[test]
    fn it_clamps_volume_adjustments_outside_of_expected_range() {
        let band_adjustments =
            VolumeAdjustments2::new([i16::MIN, i16::MAX, 0, 0, 0, 0, 0, 0].to_vec()).unwrap();
        assert_eq!(
            [
                VolumeAdjustments2::MIN_VOLUME,
                VolumeAdjustments2::MAX_VOLUME,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
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
            let actual = VolumeAdjustments2::new(example.0.to_vec())
                .unwrap()
                .apply_drc();
            let expected = VolumeAdjustments2::new(
                example.1.map(|v: f64| (v * 10f64).round() as i16).to_vec(),
            )
            .unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn it_does_not_modify_band_9_when_applying_drc() {
        let volume_adjustments =
            VolumeAdjustments2::new([-60, 60, 23, 120, 22, -120, -04, 16, 50].to_vec()).unwrap(); // volume adjustments
        let expected = VolumeAdjustments2::new(
            [
                -1.1060872, 1.367825, -0.842687, 1.571185, 0.321646, -1.79549, 0.61513, 0.083543,
                5.0,
            ]
            .map(|v| (v * 10.0f64).round() as i16)
            .to_vec(),
        )
        .unwrap(); // drc
        let actual = volume_adjustments.apply_drc();
        assert_eq!(expected, actual)
    }
}

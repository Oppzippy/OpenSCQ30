use std::array;

use crate::packets::structures::{EqualizerConfiguration, VolumeAdjustments};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerWithDrcPacket {
    left_configuration: EqualizerConfiguration,
    right_configuration: Option<EqualizerConfiguration>,
}

impl SetEqualizerWithDrcPacket {
    pub fn new(
        left_configuration: EqualizerConfiguration,
        right_configuration: Option<EqualizerConfiguration>,
    ) -> Self {
        Self {
            left_configuration,
            right_configuration,
        }
    }
}

impl OutboundPacket for SetEqualizerWithDrcPacket {
    fn command(&self) -> [u8; 7] {
        [0x08, 0xEE, 0x00, 0x00, 0x00, 0x02, 0x83]
    }

    fn body(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(34);

        // profile
        bytes.extend(self.left_configuration.profile_id().to_le_bytes());
        // eq without drc
        bytes.extend(self.left_configuration.volume_adjustments().bytes());
        if let Some(right_eq) = self.right_configuration {
            bytes.extend(right_eq.volume_adjustments().bytes());
        }
        // eq with drc
        // TODO maybe a bug? DRC is clamped to -120-120 rather than -128-127 since that's what VolumeAdjustments does.
        // The clamping is likely unnecessary anyway.
        bytes.extend(apply_drc(self.left_configuration.volume_adjustments()).bytes());
        if let Some(right_eq) = self.right_configuration {
            bytes.extend(apply_drc(right_eq.volume_adjustments()).bytes());
        }

        bytes
    }
}

fn apply_drc(volume_adjustments: VolumeAdjustments) -> VolumeAdjustments {
    const PRE_DRC_SUBTRACTION: f64 = 12.0;
    let bands = volume_adjustments
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
            } else if (band_coefficient_index == 4 || band_coefficient_index == 6) && index == 5 {
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

#[cfg(test)]
mod tests {
    use crate::packets::{
        outbound::{OutboundPacketBytes, SetEqualizerWithDrcPacket},
        structures::{EqualizerConfiguration, VolumeAdjustments},
    };

    use super::apply_drc;

    #[test]
    fn test_apply_drc() {
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
            let actual = apply_drc(VolumeAdjustments::new(example.0));
            let expected = VolumeAdjustments::new(example.1);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x83, 0x1c, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xf0, 0x8e, 0x00, 0x74, 0x88, 0xdb, 0xf0, 0xe0, 0xf0, 0xec, 0xd7, 0xef, 0xe4, 0xbd,
        ];

        let packet = SetEqualizerWithDrcPacket::new(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -60, 60, 23, 120, 22, -120, -4, 16,
            ])),
            None,
        );
        assert_eq!(EXPECTED, packet.bytes());
    }
}

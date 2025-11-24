use crate::devices::soundcore::common::{packet, structures::EqualizerConfiguration};

pub fn set_equalizer_with_drc<
    const CHANNELS: usize,
    const BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
>(
    equalizer_configuration: &EqualizerConfiguration<
        CHANNELS,
        BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >,
) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([0x02, 0x83]),
        equalizer_configuration
            .bytes()
            .chain(
                equalizer_configuration
                    .volume_adjustments()
                    .iter()
                    .flat_map(|v| v.apply_drc().bytes()),
            )
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::structures::{
        CommonEqualizerConfiguration, CommonVolumeAdjustments,
    };

    use super::*;

    #[test]
    fn it_matches_a_manually_crafted_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x83, 0x1c, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xf0, 0x8e, 0x00, 0x74, 0x88, 0x6d, 0x86, 0x70, 0x88, 0x7b, 0x66, 0x7e, 0x79, 0x4f,
        ];

        let actual = set_equalizer_with_drc(&CommonEqualizerConfiguration::new(
            0xfefe,
            [CommonVolumeAdjustments::new([
                -60, 60, 23, 120, 22, -120, -4, 16,
            ])],
        ))
        .bytes_with_checksum();
        assert_eq!(EXPECTED, actual);
    }
}

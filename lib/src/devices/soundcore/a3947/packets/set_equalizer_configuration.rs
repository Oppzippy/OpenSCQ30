use std::iter;

use crate::devices::soundcore::{
    a3947,
    common::{
        packet::{self, Command},
        structures::{CommonEqualizerConfiguration, HearIdType},
    },
};

pub fn set_equalizer_configuration<const CHANNELS: usize, const BANDS: usize>(
    equalizer_configuration: &CommonEqualizerConfiguration<CHANNELS, BANDS>,
    hear_id: &a3947::structures::HearId<CHANNELS, BANDS>,
) -> packet::Outbound {
    let active_volume_adjustments = if hear_id.is_enabled {
        if hear_id.hear_id_type == HearIdType(1) {
            &hear_id.custom_volume_adjustments
        } else {
            &hear_id.volume_adjustments
        }
    } else {
        equalizer_configuration.volume_adjustments()
    };
    let body = equalizer_configuration
        .preset_id()
        .to_le_bytes()
        .into_iter()
        .chain([hear_id.music_type.0, 0])
        .chain(
            equalizer_configuration
                .volume_adjustments()
                .iter()
                .flat_map(|v| v.bytes()),
        )
        .chain([0, 0]) // unknown
        .chain(iter::once(hear_id.is_enabled.into()))
        .chain(hear_id.volume_adjustments.iter().flat_map(|v| v.bytes()))
        .chain(hear_id.time.to_le_bytes())
        .chain(iter::once(hear_id.hear_id_type.0))
        .chain(
            hear_id
                .custom_volume_adjustments
                .iter()
                .flat_map(|v| v.bytes()),
        )
        .chain(active_volume_adjustments.iter().flat_map(|v| {
            let mut bytes = v.apply_drc().bytes();
            bytes[9] = 0;
            bytes
        }))
        .chain(iter::once(0))
        .collect();
    packet::Outbound::new(Command([3, 135]), body)
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::structures::{
        CommonVolumeAdjustments, HearIdMusicType, HearIdType,
    };

    use super::*;

    #[test]
    fn matches_known_good_packet() {
        let packet = set_equalizer_configuration(
            &CommonEqualizerConfiguration::new(
                0xfefe,
                [
                    CommonVolumeAdjustments::new([60, 0, 0, 0, 0, 0, 0, -60, 0, 0]),
                    CommonVolumeAdjustments::new([60, 0, 0, 0, 0, 0, 0, -60, 0, 0]),
                ],
            ),
            &a3947::structures::HearId {
                is_enabled: false,
                volume_adjustments: [
                    CommonVolumeAdjustments::from_bytes([
                        112, 117, 140, 148, 150, 142, 134, 131, 60, 60,
                    ]),
                    CommonVolumeAdjustments::from_bytes([
                        112, 117, 140, 148, 150, 142, 134, 131, 60, 60,
                    ]),
                ],
                time: i32::from_le_bytes([104, 100, 34, 64]),
                hear_id_type: HearIdType(2),
                music_type: HearIdMusicType(6),
                custom_volume_adjustments: [
                    CommonVolumeAdjustments::from_bytes([
                        112, 117, 140, 148, 150, 142, 134, 131, 60, 60,
                    ]),
                    CommonVolumeAdjustments::from_bytes([
                        112, 117, 140, 148, 150, 142, 134, 131, 60, 60,
                    ]),
                ],
            },
        );

        #[rustfmt::skip]
        fn expected_packet() -> Vec<u8> {
            vec![
                8, 238, 0, 0, 0, 3, 135, 103, 0,
                /* equalizer preset id */ 254, 254,
                /* hear id music type */ 6, 0,
                /* left */ 180, 120, 120, 120, 120, 120, 120, 60, 120, 120,
                /* right */ 180, 120, 120, 120, 120, 120, 120, 60, 120, 120,
                /* 3 unknown */ 0, 0, 0,
                /* left */ 112, 117, 140, 148, 150, 142, 134, 131, 60, 60,
                /* right */ 112, 117, 140, 148, 150, 142, 134, 131, 60, 60,
                /* time */ 104, 100, 34, 64,
                /* hear id type */ 2,
                /* left */ 112, 117, 140, 148, 150, 142, 134, 131, 60, 60,
                /* right */ 112, 117, 140, 148, 150, 142, 134, 131, 60, 60,
                /* left */ 128, 116, 121, 119, 121, 119, 124, 111, 120, 0,
                /*right */ 128, 116, 121, 119, 121, 119, 124, 111, 120, 0,
                /* unknown */ 0,
                143
            ]
        }

        assert_eq!(packet.bytes_with_checksum(), expected_packet())
    }
}

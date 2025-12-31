use crate::devices::soundcore::common::{
    packet,
    structures::{AgeRange, CustomHearId, EqualizerConfiguration, Gender},
};

use super::outbound_packet::ToPacket;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetEqualizerAndCustomHearId<
    'a,
    const CHANNELS: usize,
    const BANDS: usize,
    const HEAR_ID_CHANNELS: usize,
    const HEAR_ID_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> {
    pub equalizer_configuration:
        &'a EqualizerConfiguration<CHANNELS, BANDS, MIN_VOLUME, MAX_VOLUME, FRACTION_DIGITS>,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub custom_hear_id: &'a CustomHearId<HEAR_ID_CHANNELS, HEAR_ID_BANDS>,
}

impl<
    const CHANNELS: usize,
    const BANDS: usize,
    const HEAR_ID_CHANNELS: usize,
    const HEAR_ID_BANDS: usize,
    const MIN_VOLUME: i16,
    const MAX_VOLUME: i16,
    const FRACTION_DIGITS: u8,
> ToPacket
    for SetEqualizerAndCustomHearId<
        '_,
        CHANNELS,
        BANDS,
        HEAR_ID_CHANNELS,
        HEAR_ID_BANDS,
        MIN_VOLUME,
        MAX_VOLUME,
        FRACTION_DIGITS,
    >
{
    type DirectionMarker = packet::OutboundMarker;

    fn command(&self) -> packet::Command {
        // TODO does this apply to all devices?
        if self.age_range.supports_hear_id() {
            packet::Command([0x03, 0x87])
        } else {
            packet::Command([0x03, 0x86])
        }
    }

    fn body(&self) -> Vec<u8> {
        // regular eq, hear id eq, hear id custom eq, and dynamic range compression
        // plus the fixed size data
        let mut bytes = Vec::with_capacity(
            (CHANNELS * BANDS * 2) + (HEAR_ID_CHANNELS * HEAR_ID_BANDS * 2) + 12,
        );
        let supports_hear_id = self.age_range.supports_hear_id();

        let max_value_stereo_eq_wave = [255]
            .repeat(self.equalizer_configuration.channels() * self.equalizer_configuration.bands());

        bytes.extend(self.equalizer_configuration.preset_id().to_le_bytes());
        if supports_hear_id {
            bytes.extend(self.custom_hear_id.favorite_music_genre.bytes());
        }
        let eq = self.equalizer_configuration.volume_adjustments();
        bytes.extend(eq.iter().flat_map(|v| v.bytes()));
        bytes.push(if supports_hear_id {
            self.gender.0
        } else {
            u8::MAX
        });
        bytes.push(if supports_hear_id {
            self.age_range.0
        } else {
            u8::MAX
        });
        bytes.push(0); // Unknown
        if supports_hear_id {
            bytes.extend(self.custom_hear_id.volume_adjustment_bytes());
        } else {
            bytes.extend(&max_value_stereo_eq_wave);
        }
        bytes.extend(if supports_hear_id {
            self.custom_hear_id.time.to_be_bytes()
        } else {
            [0, 0, 0, 0]
        });
        bytes.push(self.custom_hear_id.hear_id_type as u8);
        if supports_hear_id {
            bytes.extend(self.custom_hear_id.custom_volume_adjustment_bytes());
        } else {
            bytes.extend(&max_value_stereo_eq_wave);
        }

        bytes.extend(
            self.equalizer_configuration
                .volume_adjustments()
                .iter()
                .flat_map(|v| v.apply_drc().bytes()),
        );

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::common::structures::{
        CommonEqualizerConfiguration, CommonVolumeAdjustments, HearIdMusicGenre, HearIdType,
    };

    use super::*;

    #[test]
    fn it_matches_a_generated_packet_with_hear_id_set() {
        let expected: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x03, 0x87, 0x56, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0x44,
            0x36, 0x38, 0x35, 0x0c, 0x62, 0x47, 0x13, 0x44, 0x36, 0x38, 0x35, 0x0c, 0x62, 0x47,
            0x13, 0x01, 0x02, 0x00, 0x35, 0x10, 0x30, 0x09, 0x4c, 0x45, 0x2c, 0x46, 0x2a, 0x10,
            0x6e, 0x19, 0x74, 0x1c, 0x28, 0x36, 0xa0, 0x86, 0x01, 0x00, 0x01, 0x0b, 0x44, 0x2f,
            0x76, 0x13, 0x04, 0x02, 0x51, 0x6c, 0x08, 0x54, 0x4f, 0x60, 0x07, 0x0e, 0x63, 0x74,
            0x74, 0x74, 0x77, 0x6b, 0x7c, 0x75, 0x6c, 0x74, 0x74, 0x74, 0x77, 0x6b, 0x7c, 0x75,
            0x6c, 0x0a,
        ];
        let actual = SetEqualizerAndCustomHearId {
            equalizer_configuration: &CommonEqualizerConfiguration::new(
                0xfefe,
                [
                    CommonVolumeAdjustments::new([-52, -66, -64, -67, -108, -22, -49, -101]),
                    CommonVolumeAdjustments::new([-52, -66, -64, -67, -108, -22, -49, -101]),
                ],
            ),
            gender: Gender(1),
            age_range: AgeRange(2),
            custom_hear_id: &CustomHearId {
                is_enabled: true,
                volume_adjustments: [
                    Some(CommonVolumeAdjustments::new([
                        -67, -104, -72, -111, -44, -51, -76, -50,
                    ])),
                    Some(CommonVolumeAdjustments::new([
                        -78, -104, -10, -95, -4, -92, -80, -66,
                    ])),
                ],
                time: 2693136640,
                hear_id_type: HearIdType::Custom,
                favorite_music_genre: HearIdMusicGenre(0xfefe),
                custom_volume_adjustments: [
                    Some(CommonVolumeAdjustments::new([
                        -109, -52, -73, -2, -101, -116, -118, -39,
                    ])),
                    Some(CommonVolumeAdjustments::new([
                        -12, -112, -36, -41, -24, -113, -106, -21,
                    ])),
                ],
            },
        }
        .to_packet()
        .bytes_with_checksum();

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_matches_a_generated_packet_with_no_hear_id() {
        let expected: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x03, 0x86, 0x54, 0x00, 0x00, 0x00, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0xff,
            0xff, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0x78, 0xb1,
        ];
        let actual = SetEqualizerAndCustomHearId {
            equalizer_configuration: &CommonEqualizerConfiguration::<2, 8>::new(
                0x0000,
                [
                    CommonVolumeAdjustments::default(),
                    CommonVolumeAdjustments::default(),
                ],
            ),
            gender: Gender(1),
            age_range: AgeRange(255),
            custom_hear_id: &CustomHearId {
                is_enabled: true,
                volume_adjustments: [
                    Some(CommonVolumeAdjustments::new([
                        -33, -1, -33, -62, -9, -99, -19, -21,
                    ])),
                    Some(CommonVolumeAdjustments::new([
                        -46, -19, -111, -2, -10, -6, -100, -101,
                    ])),
                ],
                time: 100000,
                hear_id_type: HearIdType::Initial,
                favorite_music_genre: HearIdMusicGenre(0),
                custom_volume_adjustments: [
                    Some(CommonVolumeAdjustments::new([
                        -90, -67, -53, -79, -13, -12, -73, -99,
                    ])),
                    Some(CommonVolumeAdjustments::new([
                        -24, -67, -40, -41, -38, -102, -119, -24,
                    ])),
                ],
            },
        }
        .to_packet()
        .bytes_with_checksum();

        assert_eq!(actual, expected);
    }
}

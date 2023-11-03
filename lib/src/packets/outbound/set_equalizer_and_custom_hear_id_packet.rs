use crate::packets::structures::{AgeRange, CustomHearId, EqualizerConfiguration, Gender};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetEqualizerAndCustomHearIdPacket<'a> {
    pub equalizer_configuration: &'a EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub custom_hear_id: &'a CustomHearId,
}

impl<'a> OutboundPacket for SetEqualizerAndCustomHearIdPacket<'a> {
    fn command(&self) -> [u8; 7] {
        if self.age_range.supports_hear_id() {
            [0x08, 0xee, 0x00, 0x00, 0x00, 0x03, 0x87]
        } else {
            [0x08, 0xee, 0x00, 0x00, 0x00, 0x03, 0x86]
        }
    }

    fn body(&self) -> Vec<u8> {
        // Not sure what this value means.
        // Possible values include:
        // - 0xfefe (custom)
        // - 0x0000 (if no default is set in SharedPreferences?)
        // - 0xee00 (some sort of initial value?)
        const EQ_HEAR_INDEX_ID: u16 = 0xfefe;

        const MAX_VALUE_STEREO_EQ_WAVE: [u8; 16] = [255; 16];
        let mut bytes = Vec::with_capacity(76);
        let supports_hear_id = self.age_range.supports_hear_id();

        bytes.extend(self.equalizer_configuration.profile_id().to_le_bytes());
        if supports_hear_id {
            bytes.extend(EQ_HEAR_INDEX_ID.to_le_bytes());
        }
        let eq = self.equalizer_configuration.volume_adjustments();
        bytes.extend(eq.bytes()); // left
        bytes.extend(eq.bytes()); // right
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
        bytes.push(0); // ??
        if supports_hear_id {
            bytes.extend(self.custom_hear_id.volume_adjustments.bytes());
        } else {
            bytes.extend(MAX_VALUE_STEREO_EQ_WAVE);
        }
        bytes.extend(if supports_hear_id {
            self.custom_hear_id.time.to_be_bytes()
        } else {
            [0, 0, 0, 0]
        });
        bytes.push(if supports_hear_id {
            self.custom_hear_id.hear_id_type.0
        } else {
            0
        });
        match &self.custom_hear_id.custom_volume_adjustments {
            Some(adjustments) if supports_hear_id => bytes.extend(adjustments.bytes()),
            _ => bytes.extend(MAX_VALUE_STEREO_EQ_WAVE),
        }
        let drc = self
            .equalizer_configuration
            .volume_adjustments()
            .apply_drc();
        bytes.extend(drc.bytes()); // left
        bytes.extend(drc.bytes()); // right

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        outbound::OutboundPacketBytes,
        structures::{
            AgeRange, CustomHearId, EqualizerConfiguration, Gender, HearIdMusicType, HearIdType,
            PresetEqualizerProfile, StereoVolumeAdjustments, VolumeAdjustments,
        },
    };

    use super::SetEqualizerAndCustomHearIdPacket;

    #[test]
    fn it_matches_a_generated_packet_with_hear_id_set() {
        let expected: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x03, 0x87, 0x56, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0x44,
            0x36, 0x38, 0x35, 0x0c, 0x62, 0x47, 0x13, 0x44, 0x36, 0x38, 0x35, 0x0c, 0x62, 0x47,
            0x13, 0x01, 0x02, 0x00, 0x35, 0x10, 0x30, 0x09, 0x4c, 0x45, 0x2c, 0x46, 0x2a, 0x10,
            0x6e, 0x19, 0x74, 0x1c, 0x28, 0x36, 0x00, 0x01, 0x86, 0xa0, 0x05, 0x0b, 0x44, 0x2f,
            0x76, 0x13, 0x04, 0x02, 0x51, 0x6c, 0x08, 0x54, 0x4f, 0x60, 0x07, 0x0e, 0x63, 0x74,
            0x74, 0x74, 0x77, 0x6b, 0x7c, 0x75, 0x6c, 0x74, 0x74, 0x74, 0x77, 0x6b, 0x7c, 0x75,
            0x6c, 0x0e,
        ];
        let actual = SetEqualizerAndCustomHearIdPacket {
            equalizer_configuration: &EqualizerConfiguration::new_custom_profile(
                VolumeAdjustments::new([-5.2, -6.6, -6.4, -6.7, -10.8, -2.2, -4.9, -10.1]),
            ),
            gender: Gender(1),
            age_range: AgeRange(2),
            custom_hear_id: &CustomHearId {
                is_enabled: true,
                volume_adjustments: StereoVolumeAdjustments {
                    left: VolumeAdjustments::new([
                        -6.7, -10.4, -7.2, -11.1, -4.4, -5.1, -7.6, -5.0,
                    ]),
                    right: VolumeAdjustments::new([
                        -7.8, -10.4, -1.0, -9.5, -0.4, -9.2, -8.0, -6.6,
                    ]),
                },
                time: 100000,
                hear_id_type: HearIdType(5),
                hear_id_music_type: HearIdMusicType(0),
                custom_volume_adjustments: Some(StereoVolumeAdjustments {
                    left: VolumeAdjustments::new([
                        -10.9, -5.2, -7.3, -0.2, -10.1, -11.6, -11.8, -3.9,
                    ]),
                    right: VolumeAdjustments::new([
                        -1.2, -11.2, -3.6, -4.1, -2.4, -11.3, -10.6, -2.1,
                    ]),
                }),
            },
        }
        .bytes();

        assert_eq!(expected, actual);
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
        let actual = SetEqualizerAndCustomHearIdPacket {
            equalizer_configuration: &EqualizerConfiguration::new_from_preset_profile(
                PresetEqualizerProfile::SoundcoreSignature,
            ),
            gender: Gender(1),
            age_range: AgeRange(255),
            custom_hear_id: &CustomHearId {
                is_enabled: true,
                volume_adjustments: StereoVolumeAdjustments {
                    left: VolumeAdjustments::new([-3.3, -0.1, -3.3, -6.2, -0.9, -9.9, -1.9, -2.1]),
                    right: VolumeAdjustments::new([
                        -4.6, -1.9, -11.1, -0.2, -1.0, -0.6, -10.0, -10.1,
                    ]),
                },
                time: 100000,
                hear_id_type: HearIdType(5),
                hear_id_music_type: HearIdMusicType(0),
                custom_volume_adjustments: Some(StereoVolumeAdjustments {
                    left: VolumeAdjustments::new([-9.0, -6.7, -5.3, -7.9, -1.3, -1.2, -7.3, -9.9]),
                    right: VolumeAdjustments::new([
                        -2.4, -6.7, -4.0, -4.1, -3.8, -10.2, -11.9, -2.4,
                    ]),
                }),
            },
        }
        .bytes();

        assert_eq!(expected, actual);
    }
}

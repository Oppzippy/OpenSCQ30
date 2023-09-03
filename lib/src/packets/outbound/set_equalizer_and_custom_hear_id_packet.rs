use crate::packets::structures::{AgeRange, CustomHearId, EqualizerConfiguration, Gender};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerAndCustomHearIdPacket {
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub custom_hear_id: CustomHearId,
}

impl OutboundPacket for SetEqualizerAndCustomHearIdPacket {
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
        let eq_bytes = self.equalizer_configuration.volume_adjustments().bytes();
        bytes.extend(eq_bytes); // left
        bytes.extend(eq_bytes); // right
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
        bytes.extend(if supports_hear_id {
            self.custom_hear_id.volume_adjustments.bytes()
        } else {
            MAX_VALUE_STEREO_EQ_WAVE
        });
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
        bytes.extend(if supports_hear_id {
            self.custom_hear_id
                .custom_volume_adjustments
                .map(|adjustments| adjustments.bytes())
                .unwrap_or(MAX_VALUE_STEREO_EQ_WAVE)
        } else {
            MAX_VALUE_STEREO_EQ_WAVE
        });
        let drc = self
            .equalizer_configuration
            .volume_adjustments()
            .apply_drc()
            .bytes();
        bytes.extend(drc); // left
        bytes.extend(drc); // right

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
            0x08, 0xee, 0x00, 0x00, 0x00, 0x03, 0x87, 86, 0, 0xfe, 0xfe, 0xfe, 0xfe, 0x44, 0x36,
            0x38, 0x35, 0xc, 0x62, 0x47, 0x13, 0x44, 0x36, 0x38, 0x35, 0xc, 0x62, 0x47, 0x13, 0x1,
            0x2, 0x0, 0x35, 0x10, 0x30, 0x9, 0x4c, 0x45, 0x2c, 0x46, 0x2a, 0x10, 0x6e, 0x19, 0x74,
            0x1c, 0x28, 0x36, 0x0, 0x1, 0x86, 0xa0, 0x5, 0xb, 0x44, 0x2f, 0x76, 0x13, 0x4, 0x2,
            0x51, 0x6c, 0x8, 0x54, 0x4f, 0x60, 0x7, 0xe, 0x63, 227, 230, 228, 232, 220, 237, 230,
            215, 227, 230, 228, 232, 220, 237, 230, 215, 14,
        ];
        let actual = SetEqualizerAndCustomHearIdPacket {
            equalizer_configuration: EqualizerConfiguration::new_custom_profile(
                VolumeAdjustments::new([-5.2, -6.6, -6.4, -6.7, -10.8, -2.2, -4.9, -10.1]),
            ),
            gender: Gender(1),
            age_range: AgeRange(2),
            custom_hear_id: CustomHearId {
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
            0x08, 0xee, 0x00, 0x00, 0x00, 0x03, 0x86, 84, 0, 0, 0, 120, 120, 120, 120, 120, 120,
            120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0xff, 0xff, 0, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0,
            0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 230, 234, 233, 233, 233, 233, 233, 227, 230, 234, 233, 233, 233, 233,
            233, 227, 177,
        ];
        let actual = SetEqualizerAndCustomHearIdPacket {
            equalizer_configuration: EqualizerConfiguration::new_from_preset_profile(
                PresetEqualizerProfile::SoundcoreSignature,
            ),
            gender: Gender(1),
            age_range: AgeRange(255),
            custom_hear_id: CustomHearId {
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

use crate::devices::soundcore::standard::{
    packets::Command,
    structures::{AgeRange, CustomHearId, EqualizerConfiguration, Gender},
};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, PartialEq)]
pub struct SetEqualizerAndCustomHearIdPacket<'a, const C: usize, const B: usize> {
    pub equalizer_configuration: &'a EqualizerConfiguration<C, B>,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub custom_hear_id: &'a CustomHearId<C, B>,
}

impl<const C: usize, const B: usize> OutboundPacket
    for SetEqualizerAndCustomHearIdPacket<'_, C, B>
{
    fn command(&self) -> Command {
        // TODO does this apply to all devices?
        if self.age_range.supports_hear_id() {
            Command([0x03, 0x87])
        } else {
            Command([0x03, 0x86])
        }
    }

    fn body(&self) -> Vec<u8> {
        // Not sure what this value means.
        // Possible values include:
        // - 0xfefe (custom)
        // - 0x0000 (if no default is set in SharedPreferences?)
        // - 0xee00 (some sort of initial value?)
        const EQ_HEAR_INDEX_ID: u16 = 0xfefe;

        let max_value_stereo_eq_wave = [255]
            .repeat(self.equalizer_configuration.channels() * self.equalizer_configuration.bands());
        let mut bytes = Vec::with_capacity(76);
        let supports_hear_id = self.age_range.supports_hear_id();

        bytes.extend(self.equalizer_configuration.profile_id().to_le_bytes());
        if supports_hear_id {
            bytes.extend(EQ_HEAR_INDEX_ID.to_le_bytes());
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
        bytes.push(0); // ??
        if supports_hear_id {
            bytes.extend(
                self.custom_hear_id
                    .volume_adjustments
                    .iter()
                    .flat_map(|v| v.bytes()),
            );
        } else {
            bytes.extend(&max_value_stereo_eq_wave);
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
            Some(adjustments) if supports_hear_id => {
                bytes.extend(adjustments.iter().flat_map(|v| v.bytes()))
            }
            _ => bytes.extend(&max_value_stereo_eq_wave),
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
    use crate::devices::soundcore::standard::{
        packets::outbound::OutboundPacketBytesExt,
        structures::{
            AgeRange, CustomHearId, EqualizerConfiguration, Gender, HearIdMusicType, HearIdType,
            PresetEqualizerProfile, VolumeAdjustments,
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
            equalizer_configuration: &EqualizerConfiguration::new_custom_profile([
                VolumeAdjustments::new([-52, -66, -64, -67, -108, -22, -49, -101]),
                VolumeAdjustments::new([-52, -66, -64, -67, -108, -22, -49, -101]),
            ]),
            gender: Gender(1),
            age_range: AgeRange(2),
            custom_hear_id: &CustomHearId {
                is_enabled: true,
                volume_adjustments: [
                    VolumeAdjustments::new([-67, -104, -72, -111, -44, -51, -76, -50]),
                    VolumeAdjustments::new([-78, -104, -10, -95, -04, -92, -80, -66]),
                ],
                time: 100000,
                hear_id_type: HearIdType(5),
                hear_id_music_type: HearIdMusicType(0),
                custom_volume_adjustments: Some([
                    VolumeAdjustments::new([-109, -52, -73, -02, -101, -116, -118, -39]),
                    VolumeAdjustments::new([-12, -112, -36, -41, -24, -113, -106, -21]),
                ]),
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
                [Vec::new(), Vec::new()],
            ),
            gender: Gender(1),
            age_range: AgeRange(255),
            custom_hear_id: &CustomHearId {
                is_enabled: true,
                volume_adjustments: [
                    VolumeAdjustments::new([-33, -01, -33, -62, -9, -99, -19, -21]),
                    VolumeAdjustments::new([-46, -19, -111, -2, -10, -06, -100, -101]),
                ],
                time: 100000,
                hear_id_type: HearIdType(5),
                hear_id_music_type: HearIdMusicType(0),
                custom_volume_adjustments: Some([
                    VolumeAdjustments::new([-90, -67, -53, -79, -13, -12, -73, -99]),
                    VolumeAdjustments::new([-24, -67, -40, -41, -38, -102, -119, -24]),
                ]),
            },
        }
        .bytes();

        assert_eq!(expected, actual);
    }
}

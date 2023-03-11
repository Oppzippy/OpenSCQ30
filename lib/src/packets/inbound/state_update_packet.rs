use tracing::warn;

use crate::packets::structures::{
    AmbientSoundMode, EqualizerBandOffsets, EqualizerConfiguration, NoiseCancelingMode,
    PresetEqualizerProfile,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateUpdatePacket {
    ambient_sound_mode: AmbientSoundMode,
    noise_canceling_mode: NoiseCancelingMode,
    equalizer_configuration: EqualizerConfiguration,
}

impl StateUpdatePacket {
    pub fn new(bytes: &[u8]) -> Option<StateUpdatePacket> {
        // TODO offset 9 has some meaning. it's sometimes 5, sometimes 4. maybe more values I hvaen't seen.
        const PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00];
        if bytes.starts_with(PREFIX) {
            if bytes.len() == 70 {
                let preset_profile_id = u16::from_le_bytes(bytes[11..13].try_into().unwrap());
                let equalizer_configuration =
                    match PresetEqualizerProfile::from_id(preset_profile_id) {
                        Some(preset_profile) => {
                            EqualizerConfiguration::new_from_preset_profile(preset_profile)
                        }
                        None => EqualizerConfiguration::new_custom_profile(
                            EqualizerBandOffsets::from_bytes(bytes[13..21].try_into().unwrap()),
                        ),
                    };

                let Some(ambient_sound_mode) = AmbientSoundMode::from_id(bytes[44]) else {
                    warn!("parse_state_update: invalid ambient sound mode id {}!", bytes[44]);
                    return None;
                };
                let Some(noise_canceling_mode) = NoiseCancelingMode::from_id(bytes[45]) else {
                    warn!("parse_state_update: invalid noise canceling mode id {}!", bytes[44]);
                    return None;
                };

                return Some(Self {
                    ambient_sound_mode,
                    noise_canceling_mode,
                    equalizer_configuration,
                });
            } else {
                warn!("parse_state_update: expected 70 bytes, got {}", bytes.len());
            }
        }
        None
    }

    pub fn ambient_sound_mode(&self) -> AmbientSoundMode {
        self.ambient_sound_mode
    }

    pub fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        self.noise_canceling_mode
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.equalizer_configuration
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        inbound::StateUpdatePacket,
        structures::{
            AmbientSoundMode, EqualizerBandOffsets, EqualizerConfiguration, NoiseCancelingMode,
            PresetEqualizerProfile,
        },
    };

    #[test]
    fn it_parses_packet_with_preset_eq() {
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0x01, 0x00, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let packet = StateUpdatePacket::new(PACKET_BYTES).unwrap();
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode());
        assert_eq!(NoiseCancelingMode::Transport, packet.noise_canceling_mode());
        assert_eq!(
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::Acoustic),
            packet.equalizer_configuration()
        );
    }

    #[test]
    fn it_parses_packet_with_invalid_preset_eq_id_as_a_custom_profile() {
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id 0x50 is
            //                                                                invalid
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0x50, 0x00, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let packet = StateUpdatePacket::new(PACKET_BYTES).unwrap();
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode());
        assert_eq!(NoiseCancelingMode::Transport, packet.noise_canceling_mode());
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(EqualizerBandOffsets::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            packet.equalizer_configuration()
        );
    }

    #[test]
    fn it_parses_packet_with_custom_eq() {
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let packet = StateUpdatePacket::new(PACKET_BYTES).unwrap();
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode());
        assert_eq!(NoiseCancelingMode::Transport, packet.noise_canceling_mode());
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(EqualizerBandOffsets::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            packet.equalizer_configuration()
        );
    }

    #[test]
    fn it_parses_packet_with_a_4_at_byte_offset_9() {
        // It's usually a 5 but sometimes a 4 at that byte offset. I don't know why, but it
        // doesn't seem to cause any problems.
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x04, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let packet = StateUpdatePacket::new(PACKET_BYTES).unwrap();
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode());
        assert_eq!(NoiseCancelingMode::Transport, packet.noise_canceling_mode());
        assert_eq!(
            EqualizerConfiguration::new_custom_profile(EqualizerBandOffsets::new([
                -60, 60, 23, 40, 22, 60, -4, 16
            ])),
            packet.equalizer_configuration()
        );
    }

    #[test]
    fn it_does_not_parse_invalid_ambient_sound_mode() {
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            //          valid range is 0x00 to 0x02
            0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let packet = StateUpdatePacket::new(PACKET_BYTES);
        assert_eq!(packet, None);
    }

    #[test]
    fn it_does_not_parse_invalid_noise_canceling_mode() {
        const PACKET_BYTES: &[u8] = &[
            //                                                                profile id
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            //                valid range is 0x00 to 0x02
            0x00, 0x00, 0x01, 0x03, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ];
        let packet = StateUpdatePacket::new(PACKET_BYTES);
        assert_eq!(packet, None);
    }

    #[test]
    fn it_does_not_parse_unknown_packet() {
        const PACKET_BYTES: &[u8] = &[0x01, 0x02, 0x03];
        let packet = StateUpdatePacket::new(PACKET_BYTES);
        assert_eq!(None, packet);
    }
}

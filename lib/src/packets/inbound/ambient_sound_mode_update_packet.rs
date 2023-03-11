use tracing::warn;

use crate::packets::structures::{AmbientSoundMode, NoiseCancelingMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmbientSoundModeUpdatePacket {
    ambient_sound_mode: AmbientSoundMode,
    noise_canceling_mode: NoiseCancelingMode,
}

impl AmbientSoundModeUpdatePacket {
    pub fn new(bytes: &[u8]) -> Option<Self> {
        const PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00];
        if bytes.starts_with(PREFIX) {
            if bytes.len() == 14 {
                let ambient_sound_mode_id = bytes[9];
                let noise_canceling_mode_id = bytes[10];
                let Some(ambient_sound_mode) = AmbientSoundMode::from_id(ambient_sound_mode_id) else {
                    warn!("parse_ambient_sound_mode_update: invalid ambient sound mode id {}", ambient_sound_mode_id);
                    return None;
                };
                let Some(noise_canceling_mode) = NoiseCancelingMode::from_id(noise_canceling_mode_id) else {
                    warn!("parse_noise_canceling_mode_update: invalid noise canceling mode id {}", noise_canceling_mode_id);
                    return None;
                };

                return Some(Self {
                    ambient_sound_mode,
                    noise_canceling_mode,
                });
            } else {
                warn!(
                    "parse_noise_canceling_mode_update: expected 14 bytes, got {}",
                    bytes.len()
                );
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
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        inbound::AmbientSoundModeUpdatePacket,
        structures::{AmbientSoundMode, NoiseCancelingMode},
    };

    #[test]
    fn it_parses_valid_packet() {
        const PACKET_BYTES: &[u8] = &[
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x02, 0x02, 0x01, 0x00, 0x23,
        ];
        let packet = AmbientSoundModeUpdatePacket::new(PACKET_BYTES).unwrap();
        assert_eq!(AmbientSoundMode::Normal, packet.ambient_sound_mode());
        assert_eq!(NoiseCancelingMode::Indoor, packet.noise_canceling_mode());
    }

    #[test]
    fn it_does_not_parse_invalid_ambient_sound_mode() {
        const PACKET_BYTES: &[u8] = &[
            //                                                    max value of 0x02
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x03, 0x02, 0x01, 0x00, 0x23,
        ];
        let packet = AmbientSoundModeUpdatePacket::new(PACKET_BYTES);
        assert_eq!(None, packet);
    }

    #[test]
    fn it_does_not_parse_invalid_noise_canceling_mode() {
        const PACKET_BYTES: &[u8] = &[
            //                                                          max value of 0x02
            0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x02, 0x03, 0x01, 0x00, 0x23,
        ];
        let packet = AmbientSoundModeUpdatePacket::new(PACKET_BYTES);
        assert_eq!(None, packet);
    }

    #[test]
    fn it_does_not_parse_unknown_packet() {
        const PACKET_BYTES: &[u8] = &[0x01, 0x02, 0x03];
        let packet = AmbientSoundModeUpdatePacket::new(PACKET_BYTES);
        assert_eq!(None, packet);
    }
}

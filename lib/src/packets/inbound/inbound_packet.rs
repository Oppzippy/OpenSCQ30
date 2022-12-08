use tracing::warn;

use crate::packets::structures::{
    ambient_sound_mode::AmbientSoundMode, equalizer_band_offsets::EqualizerBandOffsets,
    equalizer_configuration::EqualizerConfiguration, noise_canceling_mode::NoiseCancelingMode,
    preset_equalizer_profile::PresetEqualizerProfile,
};

pub enum InboundPacket {
    StateUpdate {
        ambient_sound_mode: AmbientSoundMode,
        noise_canceling_mode: NoiseCancelingMode,
        equalizer_configuration: EqualizerConfiguration,
    },
    AmbientSoundModeUpdate {
        ambient_sound_mode: AmbientSoundMode,
        noise_canceling_mode: NoiseCancelingMode,
    },
}

impl InboundPacket {
    pub fn from_bytes(bytes: &[u8]) -> Option<InboundPacket> {
        Self::parse_state_update(bytes).or_else(|| Self::parse_ambient_sound_mode_update(bytes))
    }

    fn parse_state_update(bytes: &[u8]) -> Option<InboundPacket> {
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

                return Some(Self::StateUpdate {
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

    fn parse_ambient_sound_mode_update(bytes: &[u8]) -> Option<InboundPacket> {
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

                return Some(Self::AmbientSoundModeUpdate {
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
}

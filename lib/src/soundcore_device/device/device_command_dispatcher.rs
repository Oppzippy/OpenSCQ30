use crate::devices::standard::{
    packets::outbound::{
        OutboundPacketBytes, SetCustomButtonModelPacket, SetEqualizerAndCustomHearIdPacket,
        SetEqualizerPacket, SetEqualizerWithDrcPacket, SetSoundModePacket,
    },
    state::DeviceState,
    structures::{
        AmbientSoundMode, CustomButtonModel, CustomHearId, EqualizerConfiguration, HearId,
        HearIdMusicType, HearIdType, SoundModes,
    },
};

use super::soundcore_command::CommandResponse;

pub struct DefaultDispatcher;
impl DeviceCommandDispatcher for DefaultDispatcher {}

pub trait DeviceCommandDispatcher {
    fn set_sound_modes(
        &self,
        state: DeviceState,
        sound_modes: SoundModes,
    ) -> crate::Result<CommandResponse> {
        let Some(prev_sound_modes) = state.sound_modes else {
            return Err(crate::Error::MissingData {
                name: "sound modes",
            });
        };

        let mut packets = Vec::new();
        // It will bug and put us in noise canceling mode without changing the ambient sound mode id if we change the
        // noise canceling mode with the ambient sound mode being normal or transparency. To work around this, we must
        // set the ambient sound mode to Noise Canceling, and then change it back.
        let needs_noise_canceling = prev_sound_modes.ambient_sound_mode
            != AmbientSoundMode::NoiseCanceling
            && prev_sound_modes.noise_canceling_mode != sound_modes.noise_canceling_mode;
        if needs_noise_canceling {
            packets.push(
                SetSoundModePacket {
                    ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
                    noise_canceling_mode: prev_sound_modes.noise_canceling_mode,
                    transparency_mode: prev_sound_modes.transparency_mode,
                    custom_noise_canceling: prev_sound_modes.custom_noise_canceling,
                }
                .bytes(),
            );
        }

        // If we need to temporarily be in noise canceling mode to work around the bug, set all fields besides
        // ambient_sound_mode. Otherwise, we set all fields in one go.
        packets.push(
            SetSoundModePacket {
                ambient_sound_mode: if needs_noise_canceling {
                    AmbientSoundMode::NoiseCanceling
                } else {
                    sound_modes.ambient_sound_mode
                },
                noise_canceling_mode: sound_modes.noise_canceling_mode,
                transparency_mode: sound_modes.transparency_mode,
                custom_noise_canceling: sound_modes.custom_noise_canceling,
            }
            .bytes(),
        );

        // Switch to the target sound mode if we didn't do it in the previous step.
        // If the target sound mode is noise canceling, we already set it to that, so no change needed.
        if needs_noise_canceling
            && sound_modes.ambient_sound_mode != AmbientSoundMode::NoiseCanceling
        {
            packets.push(
                SetSoundModePacket {
                    ambient_sound_mode: sound_modes.ambient_sound_mode,
                    noise_canceling_mode: sound_modes.noise_canceling_mode,
                    transparency_mode: sound_modes.transparency_mode,
                    custom_noise_canceling: sound_modes.custom_noise_canceling,
                }
                .bytes(),
            );
        }

        Ok(CommandResponse {
            packets,
            new_state: DeviceState {
                sound_modes: Some(sound_modes),
                ..state
            },
        })
    }

    fn set_equalizer_configuration(
        &self,
        state: DeviceState,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<CommandResponse> {
        let left_channel = &equalizer_configuration;
        let right_channel = if state.device_profile.num_equalizer_channels == 2 {
            Some(&equalizer_configuration)
        } else {
            None
        };

        let packet_bytes = if let Some(HearId::Custom(custom_hear_id)) = &state.hear_id {
            SetEqualizerAndCustomHearIdPacket {
                equalizer_configuration: &equalizer_configuration,
                age_range: state.age_range.ok_or(crate::Error::IncompleteStateError {
                    message: "age range not set",
                })?,
                custom_hear_id,
                gender: state.gender.ok_or(crate::Error::IncompleteStateError {
                    message: "gender not set",
                })?,
            }
            .bytes()
        } else if state.supports_dynamic_range_compression() {
            SetEqualizerWithDrcPacket::new(left_channel, right_channel).bytes()
        } else {
            SetEqualizerPacket::new(left_channel, right_channel).bytes()
        };
        Ok(CommandResponse {
            packets: vec![packet_bytes],
            new_state: DeviceState {
                equalizer_configuration,
                ..state
            },
        })
    }

    fn set_hear_id(&self, state: DeviceState, hear_id: HearId) -> crate::Result<CommandResponse> {
        fn set_custom_hear_id(
            state: &DeviceState,
            custom_hear_id: &CustomHearId,
        ) -> crate::Result<Vec<u8>> {
            let gender = state
                .gender
                .ok_or(crate::Error::MissingData { name: "gender" })?;
            let age_range = state
                .age_range
                .ok_or(crate::Error::MissingData { name: "age range" })?;
            let packet = SetEqualizerAndCustomHearIdPacket {
                equalizer_configuration: &state.equalizer_configuration,
                gender,
                age_range,
                custom_hear_id,
            };
            Ok(packet.bytes())
        }

        let packet = match &hear_id {
            HearId::Basic(hear_id) => {
                set_custom_hear_id(
                    &state,
                    &CustomHearId {
                        is_enabled: hear_id.is_enabled,
                        volume_adjustments: hear_id.volume_adjustments.to_owned(),
                        // TODO Should this be the current time? If so, what kind of timestamp?
                        time: hear_id.time,
                        hear_id_type: HearIdType::default(),
                        hear_id_music_type: HearIdMusicType::default(),
                        custom_volume_adjustments: None,
                    },
                )
            }
            HearId::Custom(hear_id) => set_custom_hear_id(&state, hear_id),
        }?;

        Ok(CommandResponse {
            packets: vec![packet],
            new_state: DeviceState {
                hear_id: Some(hear_id),
                ..state
            },
        })
    }

    fn set_custom_button_model(
        &self,
        state: DeviceState,
        custom_button_model: CustomButtonModel,
    ) -> crate::Result<CommandResponse> {
        if !state.device_profile.has_custom_button_model {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "custom button model",
            });
        }

        let prev_custom_button_model =
            state.custom_button_model.ok_or(crate::Error::MissingData {
                name: "custom button model",
            })?;
        if custom_button_model == prev_custom_button_model {
            return Ok(CommandResponse {
                packets: Vec::new(),
                new_state: state,
            });
        }

        let packet = SetCustomButtonModelPacket::new(custom_button_model).bytes();
        Ok(CommandResponse {
            packets: vec![packet],
            new_state: DeviceState {
                custom_button_model: Some(custom_button_model),
                ..state
            },
        })
    }
}

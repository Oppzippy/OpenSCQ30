use std::{collections::HashMap, sync::Arc};

use nom::error::VerboseError;

use crate::{
    device_profile::{DeviceFeatures, DeviceProfile},
    devices::standard::{
        self,
        packets::inbound::{
            state_update_packet::StateUpdatePacket, InboundPacket, SoundModeTypeTwoUpdatePacket,
        },
        quirks::{TwoExtraEqBandSetEqualizerPacket, TwoExtraEqBands},
        state::DeviceState,
        structures::*,
    },
    soundcore_device::{
        device::{device_implementation::DeviceImplementation, soundcore_command::CommandResponse},
        device_model::DeviceModel,
    },
};

use super::packets::{A3936SetCustomButtonModelPacket, A3936StateUpdatePacket};

pub(crate) const A3936_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        sound_mode: None,
        has_hear_id: true,
        num_equalizer_channels: 2,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: true,
        dynamic_range_compression_min_firmware_version: None,
        has_custom_button_model: true,
        has_wear_detection: false,
        has_touch_tone: false,
        has_auto_power_off: false,
        has_ambient_sound_mode_cycle: true,
    },
    compatible_models: &[DeviceModel::A3936],
    implementation: || Arc::new(A3936Implementation::default()),
};

#[derive(Debug, Default)]
pub struct A3936Implementation {
    // The official app only displays 8 bands, so I have no idea what bands 9 and 10 do. We'll just keep track
    // of their initial value and resend that.
    extra_bands: Arc<TwoExtraEqBands>,
}

impl DeviceImplementation for A3936Implementation {
    fn packet_handlers(
        &self,
    ) -> HashMap<Command, Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
        let extra_bands = self.extra_bands.to_owned();
        let mut handlers = standard::implementation::packet_handlers();

        handlers.insert(
            STATE_UPDATE,
            Box::new(move |packet_bytes, state| {
                let result = A3936StateUpdatePacket::take::<VerboseError<_>>(packet_bytes);
                let packet = match result {
                    Ok((_, packet)) => packet,
                    Err(err) => {
                        tracing::error!("failed to parse packet: {err:?}");
                        return state;
                    }
                };
                extra_bands.set_values(packet.extra_bands);

                StateUpdatePacket::from(packet).into()
            }),
        );
        handlers.insert(
            SOUND_MODE_UPDATE,
            Box::new(move |packet_bytes, state| {
                let packet =
                    match SoundModeTypeTwoUpdatePacket::take::<VerboseError<_>>(packet_bytes) {
                        Ok((_, packet)) => packet,
                        Err(err) => {
                            tracing::error!("failed to parse packet: {err:?}");
                            return state;
                        }
                    };
                DeviceState {
                    sound_modes_type_two: Some(packet.sound_modes),
                    ..state
                }
            }),
        );

        handlers
    }

    fn initialize(&self, packet: &[u8]) -> crate::Result<DeviceState> {
        let packet = A3936StateUpdatePacket::take::<VerboseError<_>>(packet)
            .map(|(_, packet)| packet)
            .map_err(|err| crate::Error::ParseError {
                message: format!("{err:?}"),
            })?;
        Ok(StateUpdatePacket::from(packet).into())
    }

    fn set_equalizer_configuration(
        &self,
        state: DeviceState,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<CommandResponse> {
        let left_channel = &equalizer_configuration;
        let right_channel = &equalizer_configuration;
        let extra_band_values = self.extra_bands.values();

        let packet = TwoExtraEqBandSetEqualizerPacket {
            left_channel,
            right_channel,
            extra_band_values,
        };

        Ok(CommandResponse {
            packets: vec![packet.into()],
            new_state: DeviceState {
                equalizer_configuration,
                ..state
            },
        })
    }

    fn set_sound_modes(
        &self,
        state: DeviceState,
        sound_modes: SoundModes,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_sound_modes(state, sound_modes)
    }

    fn set_sound_modes_type_two(
        &self,
        state: DeviceState,
        sound_modes: SoundModesTypeTwo,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_sound_modes_type_two(state, sound_modes)
    }

    fn set_hear_id(&self, state: DeviceState, hear_id: HearId) -> crate::Result<CommandResponse> {
        standard::implementation::set_hear_id(state, hear_id)
    }

    fn set_custom_button_model(
        &self,
        state: DeviceState,
        custom_button_model: CustomButtonModel,
    ) -> crate::Result<CommandResponse> {
        if !state.device_features.has_custom_button_model {
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

        let packet = A3936SetCustomButtonModelPacket::new(custom_button_model.into());
        Ok(CommandResponse {
            packets: vec![packet.into()],
            new_state: DeviceState {
                custom_button_model: Some(custom_button_model),
                ..state
            },
        })
    }

    fn set_ambient_sound_mode_cycle(
        &self,
        state: DeviceState,
        cycle: AmbientSoundModeCycle,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_ambient_sound_mode_cycle(state, cycle)
    }
}

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use nom::error::VerboseError;

use crate::{
    device_profile::{DeviceFeatures, DeviceProfile},
    devices::standard::{
        self,
        macros::soundcore_device,
        packets::inbound::{InboundPacket, state_update_packet::StateUpdatePacket},
        quirks::{TwoExtraEqBandSetEqualizerPacket, TwoExtraEqBands},
        state::DeviceState,
        structures::*,
    },
    soundcore_device::{
        device::{device_implementation::DeviceImplementation, soundcore_command::CommandResponse},
        device_model::DeviceModel,
    },
};

use super::{
    packets::{
        A3936SetMultiButtonConfigurationPacket, A3936SoundModesUpdatePacket, A3936StateUpdatePacket,
    },
    state::A3936State,
    structures::{A3936InternalMultiButtonConfiguration, A3936SoundModes, A3936TwsButtonAction},
};

pub(crate) const A3936_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        available_sound_modes: None,
        has_hear_id: true,
        num_equalizer_channels: 2,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: true,
        dynamic_range_compression_min_firmware_version: None,
        has_button_configuration: true,
        has_wear_detection: false,
        has_touch_tone: false,
        has_auto_power_off: false,
        has_ambient_sound_mode_cycle: true,
    },
    compatible_models: &[DeviceModel::SoundcoreA3936],
    implementation: || Arc::new(A3936Implementation::default()),
};

#[derive(Debug, Default)]
struct A3936Implementation {
    // The official app only displays 8 bands, so I have no idea what bands 9 and 10 do. We'll just keep track
    // of their initial value and resend that.
    extra_bands: Arc<TwoExtraEqBands>,
    buttons: Arc<A3936ButtonConfigurationImplementation>,
}

impl DeviceImplementation for A3936Implementation {
    fn packet_handlers(
        &self,
    ) -> HashMap<Command, Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
        let extra_bands = self.extra_bands.to_owned();
        let buttons = self.buttons.to_owned();
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
                // extra_bands.set_values(packet.extra_bands);
                buttons.set_internal_data(packet.button_configuration);

                StateUpdatePacket::from(packet).into()
            }),
        );
        handlers.insert(
            SOUND_MODE_UPDATE,
            Box::new(move |packet_bytes, state| {
                let packet =
                    match A3936SoundModesUpdatePacket::take::<VerboseError<_>>(packet_bytes) {
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
        sound_modes: A3936SoundModes,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_sound_modes_type_two(state, sound_modes)
    }

    fn set_hear_id(&self, state: DeviceState, hear_id: HearId) -> crate::Result<CommandResponse> {
        standard::implementation::set_hear_id(state, hear_id)
    }

    fn set_multi_button_configuration(
        &self,
        state: DeviceState,
        button_configuration: MultiButtonConfiguration,
    ) -> crate::Result<CommandResponse> {
        if !state.device_features.has_button_configuration {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "custom button model",
            });
        }
        let Some(tws_status) = state.tws_status else {
            return Err(crate::Error::MissingData { name: "tws status" });
        };

        let prev_button_configuration =
            state
                .button_configuration
                .ok_or(crate::Error::MissingData {
                    name: "custom button model",
                })?;
        if button_configuration == prev_button_configuration {
            return Ok(CommandResponse {
                packets: Vec::new(),
                new_state: state,
            });
        }

        let packet = A3936SetMultiButtonConfigurationPacket::new(
            self.buttons
                .set_multi_button_configuration(&tws_status, button_configuration),
        );
        Ok(CommandResponse {
            packets: vec![packet.into()],
            new_state: DeviceState {
                button_configuration: Some(button_configuration),
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

#[derive(Debug, Default)]
pub struct A3936ButtonConfigurationImplementation {
    data: Mutex<Option<A3936InternalMultiButtonConfiguration>>,
}

impl A3936ButtonConfigurationImplementation {
    pub fn set_multi_button_configuration(
        &self,
        tws_status: &TwsStatus,
        actions: MultiButtonConfiguration,
    ) -> A3936InternalMultiButtonConfiguration {
        let mut model = self
            .data
            .lock()
            .unwrap()
            .expect("internal data should be set during initialization");

        let is_tws_connected = tws_status.is_connected;
        [
            (&mut model.left_single_click, actions.left_single_click),
            (&mut model.left_double_click, actions.left_double_click),
            (&mut model.left_long_press, actions.left_long_press),
            (&mut model.right_single_click, actions.left_single_click),
            (&mut model.right_double_click, actions.left_double_click),
            (&mut model.right_long_press, actions.left_long_press),
        ]
        .into_iter()
        .for_each(|(m, state)| Self::set_button(m, state, is_tws_connected));

        model
    }

    fn set_button(
        button: &mut A3936TwsButtonAction,
        state: ButtonConfiguration,
        is_tws_connected: bool,
    ) {
        button.set_action(state.action, is_tws_connected);
        button.set_enabled(state.is_enabled, is_tws_connected);
    }

    pub fn set_internal_data(&self, data: A3936InternalMultiButtonConfiguration) {
        *self.data.lock().unwrap() = Some(data);
    }
}

soundcore_device!(A3936State, A3936StateUpdatePacket, async |builder| {
    builder.module_collection().add_state_update();
    builder.a3936_sound_modes();
    builder.stereo_equalizer_with_custom_hear_id().await;
    builder.a3936_button_configuration();
    builder.ambient_sound_mode_cycle();
});

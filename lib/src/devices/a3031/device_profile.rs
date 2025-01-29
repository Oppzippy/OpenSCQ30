use std::{collections::HashMap, sync::Arc};

use nom::error::VerboseError;

use crate::{
    device_profile::{AvailableSoundModes, DeviceFeatures, DeviceProfile},
    devices::standard::{
        self,
        implementation::ButtonConfigurationImplementation,
        packets::inbound::{state_update_packet::StateUpdatePacket, InboundPacket},
        state::DeviceState,
        structures::{
            AmbientSoundMode, AmbientSoundModeCycle, Command, EqualizerConfiguration, HearId,
            MultiButtonConfiguration, NoiseCancelingMode, SoundModes, SoundModesTypeTwo,
            STATE_UPDATE,
        },
    },
    soundcore_device::{
        device::{device_implementation::DeviceImplementation, soundcore_command::CommandResponse},
        device_model::DeviceModel,
    },
};

use super::packets::A3031StateUpdatePacket;

pub(crate) const A3031_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        available_sound_modes: Some(AvailableSoundModes {
            ambient_sound_modes: &[
                AmbientSoundMode::Normal,
                AmbientSoundMode::Transparency,
                AmbientSoundMode::NoiseCanceling,
            ],
            transparency_modes: &[],
            noise_canceling_modes: &[
                NoiseCancelingMode::Transport,
                NoiseCancelingMode::Indoor,
                NoiseCancelingMode::Outdoor,
            ],
            custom_noise_canceling: false,
        }),
        has_hear_id: false,
        num_equalizer_channels: 1,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: false,
        dynamic_range_compression_min_firmware_version: None,
        has_button_configuration: true,
        has_wear_detection: false,
        has_touch_tone: true,
        has_auto_power_off: true,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::SoundcoreA3031],
    implementation: || Arc::new(A3031Implementation::default()),
};

#[derive(Debug, Default)]
struct A3031Implementation {
    buttons: Arc<ButtonConfigurationImplementation>,
}

impl DeviceImplementation for A3031Implementation {
    fn packet_handlers(
        &self,
    ) -> HashMap<Command, Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
        let mut handlers = standard::implementation::packet_handlers();
        let buttons = self.buttons.to_owned();

        handlers.insert(
            STATE_UPDATE,
            Box::new(move |packet_bytes, state| {
                let packet = match A3031StateUpdatePacket::take::<VerboseError<_>>(packet_bytes) {
                    Ok((_, packet)) => packet,
                    Err(err) => {
                        tracing::error!("failed to parse packet: {err:?}");
                        return state;
                    }
                };
                buttons.set_internal_data(packet.button_configuration);

                StateUpdatePacket::from(packet).into()
            }),
        );

        handlers
    }

    fn initialize(&self, packet: &[u8]) -> crate::Result<DeviceState> {
        let packet = A3031StateUpdatePacket::take::<VerboseError<_>>(packet)
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
        standard::implementation::set_equalizer_configuration(state, equalizer_configuration)
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

    fn set_multi_button_configuration(
        &self,
        state: DeviceState,
        button_configuration: MultiButtonConfiguration,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_multi_button_configuration(
            state,
            &self.buttons,
            button_configuration,
        )
    }

    fn set_ambient_sound_mode_cycle(
        &self,
        state: DeviceState,
        cycle: AmbientSoundModeCycle,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_ambient_sound_mode_cycle(state, cycle)
    }
}

use std::{collections::HashMap, sync::Arc};

use nom::error::VerboseError;

use crate::{
    device_profile::{
        DeviceFeatures, DeviceProfile, NoiseCancelingModeType, SoundModeProfile,
        TransparencyModeType,
    },
    devices::standard::{
        self,
        implementation::CustomButtonModelImplementation,
        packets::inbound::{state_update_packet::StateUpdatePacket, InboundPacket},
        state::DeviceState,
        structures::{
            AmbientSoundModeCycle, Command, CustomButtonActions, EqualizerConfiguration,
            FirmwareVersion, HearId, SoundModes, SoundModesTypeTwo, STATE_UPDATE,
        },
    },
    soundcore_device::{
        device::{device_implementation::DeviceImplementation, soundcore_command::CommandResponse},
        device_model::DeviceModel,
    },
};

use super::packets::A3931StateUpdatePacket;

pub(crate) const A3931_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        sound_mode: Some(SoundModeProfile {
            noise_canceling_mode_type: NoiseCancelingModeType::None,
            transparency_mode_type: TransparencyModeType::Custom,
        }),
        has_hear_id: false,
        num_equalizer_channels: 2,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: true,
        dynamic_range_compression_min_firmware_version: Some(FirmwareVersion::new(2, 0)),
        has_custom_button_model: true,
        has_wear_detection: false,
        has_touch_tone: true,
        has_auto_power_off: true,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::A3931, DeviceModel::A3935],
    implementation: || Arc::new(A3931Implementation::default()),
};

#[derive(Debug, Default)]
struct A3931Implementation {
    buttons: Arc<CustomButtonModelImplementation>,
}

impl DeviceImplementation for A3931Implementation {
    fn packet_handlers(
        &self,
    ) -> HashMap<Command, Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
        let mut handlers = standard::implementation::packet_handlers();
        let buttons = self.buttons.to_owned();

        handlers.insert(
            STATE_UPDATE,
            Box::new(move |packet_bytes, state| {
                let packet = match A3931StateUpdatePacket::take::<VerboseError<_>>(packet_bytes) {
                    Ok((_, packet)) => packet,
                    Err(err) => {
                        tracing::error!("failed to parse packet: {err:?}");
                        return state;
                    }
                };
                buttons.set_internal_data(packet.custom_button_model);

                StateUpdatePacket::from(packet).into()
            }),
        );

        handlers
    }

    fn initialize(&self, packet: &[u8]) -> crate::Result<DeviceState> {
        let packet = A3931StateUpdatePacket::take::<VerboseError<_>>(packet)
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

    fn set_custom_button_actions(
        &self,
        state: DeviceState,
        custom_button_model: CustomButtonActions,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_custom_button_model(state, &self.buttons, custom_button_model)
    }

    fn set_ambient_sound_mode_cycle(
        &self,
        state: DeviceState,
        cycle: AmbientSoundModeCycle,
    ) -> crate::Result<CommandResponse> {
        standard::implementation::set_ambient_sound_mode_cycle(state, cycle)
    }
}

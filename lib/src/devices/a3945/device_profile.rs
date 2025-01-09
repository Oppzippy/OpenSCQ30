use std::{collections::HashMap, sync::Arc};

use nom::error::VerboseError;

use crate::{
    device_profile::{DeviceFeatures, DeviceProfile},
    devices::standard::{
        self,
        implementation::ButtonConfigurationImplementation,
        packets::inbound::state_update_packet::StateUpdatePacket,
        quirks::{TwoExtraEqBandSetEqualizerPacket, TwoExtraEqBands},
        state::DeviceState,
        structures::*,
    },
    soundcore_device::{
        device::{device_implementation::DeviceImplementation, soundcore_command::CommandResponse},
        device_model::DeviceModel,
    },
};

use super::packets::A3945StateUpdatePacket;

pub(crate) const A3945_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    features: DeviceFeatures {
        available_sound_modes: None,
        has_hear_id: false,
        num_equalizer_channels: 2,
        num_equalizer_bands: 8,
        has_dynamic_range_compression: false,
        dynamic_range_compression_min_firmware_version: None,
        has_button_configuration: true,
        has_wear_detection: false,
        has_touch_tone: false,
        has_auto_power_off: false,
        has_ambient_sound_mode_cycle: false,
    },
    compatible_models: &[DeviceModel::A3945],
    implementation: || Arc::new(A3945Implementation::default()),
};

#[derive(Debug, Default)]
struct A3945Implementation {
    // The official app only displays 8 bands, so I have no idea what bands 9 and 10 do. We'll just keep track
    // of their initial value and resend that.
    extra_bands: Arc<TwoExtraEqBands>,
    buttons: Arc<ButtonConfigurationImplementation>,
}

impl DeviceImplementation for A3945Implementation {
    fn packet_handlers(
        &self,
    ) -> HashMap<Command, Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
        let extra_bands = self.extra_bands.to_owned();
        let buttons = self.buttons.to_owned();
        let mut handlers = standard::implementation::packet_handlers();

        handlers.insert(
            STATE_UPDATE,
            Box::new(move |packet_bytes, state| {
                let packet = match A3945StateUpdatePacket::take::<VerboseError<_>>(packet_bytes) {
                    Ok((_, packet)) => packet,
                    Err(err) => {
                        tracing::error!("failed to parse packet: {err:?}");
                        return state;
                    }
                };
                extra_bands.set_values(packet.extra_band_values);
                buttons.set_internal_data(packet.button_configuration);

                StateUpdatePacket::from(packet).into()
            }),
        );

        handlers
    }

    fn initialize(&self, packet: &[u8]) -> crate::Result<DeviceState> {
        let packet = A3945StateUpdatePacket::take::<VerboseError<_>>(packet)
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

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::{
        devices::{
            a3945::packets::A3945StateUpdatePacket,
            standard::{
                packets::{
                    inbound::{state_update_packet::StateUpdatePacket, take_inbound_packet_header},
                    outbound::{OutboundPacket, OutboundPacketBytesExt},
                },
                quirks::{TwoExtraEqBandSetEqualizerPacket, TwoExtraEqBandsValues},
                state::DeviceState,
                structures::{EqualizerConfiguration, PresetEqualizerProfile, STATE_UPDATE},
            },
        },
        soundcore_device::device::Packet,
    };

    use super::{Command, A3945_DEVICE_PROFILE};

    struct A3945TestStateUpdatePacket {
        body: Vec<u8>,
    }
    impl OutboundPacket for A3945TestStateUpdatePacket {
        fn command(&self) -> Command {
            STATE_UPDATE
        }

        fn body(&self) -> Vec<u8> {
            self.body.to_owned()
        }
    }

    #[test]
    fn it_remembers_band_9_and_10_values() {
        let data = A3945TestStateUpdatePacket {
            body: vec![
                0x01, // host device
                0x00, // tws status
                0x00, 0x00, 0x00, 0x00, // dual battery
                b'0', b'0', b'.', b'0', b'0', // left firmware version
                b'0', b'0', b'.', b'0', b'0', // right firmware version
                b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
                b'0', b'0', // serial number
                0x00, 0x00, // eq profile id
                120, 120, 120, 120, 120, 120, 120, 120, 121, 122, // left eq
                120, 120, 120, 120, 120, 120, 120, 120, 123, 124, // right eq
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, // custom button model
                0x00, // tone switch
                0x00, // wear detection
                0x00, // gaming mode
                0x00, // case battery
                0x00, // bass up
                0x00, // device color
            ],
        }
        .bytes();

        let body = take_inbound_packet_header::<VerboseError<_>>(&data)
            .unwrap()
            .0;
        let state_update = A3945StateUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        let state: DeviceState = StateUpdatePacket::from(state_update).into();
        let implementation = (A3945_DEVICE_PROFILE.implementation)();
        let state = implementation.packet_handlers()[&STATE_UPDATE](body, state);

        let equalizer_configuration =
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::TrebleReducer);
        let command_response = implementation
            .set_equalizer_configuration(state, equalizer_configuration.to_owned())
            .unwrap();

        assert_eq!(1, command_response.packets.len());
        assert_eq!(
            &Packet::from(TwoExtraEqBandSetEqualizerPacket {
                left_channel: &equalizer_configuration,
                right_channel: &equalizer_configuration,
                extra_band_values: TwoExtraEqBandsValues {
                    left_extra_1: 121,
                    left_extra_2: 122,
                    right_extra_1: 123,
                    right_extra_2: 124
                },
            }),
            command_response.packets.first().unwrap(),
        );
    }
}

use std::{collections::HashMap, sync::Arc};

use nom::error::VerboseError;

use crate::{
    device_profiles::{
        DeviceProfile, NoiseCancelingModeType, SoundModeProfile, TransparencyModeType,
    },
    devices::standard::{
        packets::outbound::OutboundPacketBytes,
        quirks::{TwoExtraEqBandSetEqualizerPacket, TwoExtraEqBands},
        state::DeviceState,
        structures::{EqualizerConfiguration, STATE_UPDATE},
    },
    soundcore_device::device::{
        device_command_dispatcher::DeviceCommandDispatcher,
        packet_handlers::state_update::state_update_handler, soundcore_command::CommandResponse,
    },
};

use super::packets::inbound::A3933StateUpdatePacket;

pub const A3933_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    sound_mode: Some(SoundModeProfile {
        noise_canceling_mode_type: NoiseCancelingModeType::Basic,
        transparency_mode_type: TransparencyModeType::Custom,
    }),
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
    custom_dispatchers: Some(|| Arc::new(A3933Dispatcher::default())),
};

#[derive(Debug, Default)]
pub struct A3933Dispatcher {
    // The official app only displays 8 bands, so I have no idea what bands 9 and 10 do. We'll just keep track
    // of their initial value and resend that.
    extra_bands: Arc<TwoExtraEqBands>,
}

impl DeviceCommandDispatcher for A3933Dispatcher {
    fn packet_handlers(
        &self,
    ) -> HashMap<[u8; 7], Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
        let extra_bands = self.extra_bands.to_owned();
        let mut handlers: HashMap<
            [u8; 7],
            Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>,
        > = HashMap::new();

        handlers.insert(
            STATE_UPDATE,
            Box::new(move |packet_bytes, state| {
                let result = A3933StateUpdatePacket::take::<VerboseError<_>>(packet_bytes);
                let packet = match result {
                    Ok((_, packet)) => packet,
                    Err(err) => {
                        tracing::error!("failed to parse packet: {err:?}");
                        return state;
                    }
                };
                extra_bands.set_values(packet.extra_band_values);

                // We only needed to capture information. The actual state transformation is passed on to the default handler..
                state_update_handler(packet_bytes, state)
            }),
        );

        handlers
    }

    fn set_equalizer_configuration(
        &self,
        state: DeviceState,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<CommandResponse> {
        let left_channel = &equalizer_configuration;
        let right_channel = &equalizer_configuration;
        let extra_band_values = self.extra_bands.values();

        let packet_bytes = TwoExtraEqBandSetEqualizerPacket {
            left_channel,
            right_channel,
            extra_band_values,
        }
        .bytes();

        Ok(CommandResponse {
            packets: vec![packet_bytes],
            new_state: DeviceState {
                equalizer_configuration,
                ..state
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::{
        a3933::{device_profile::A3933_DEVICE_PROFILE, packets::inbound::A3933StateUpdatePacket},
        standard::{
            packets::{
                inbound::{state_update_packet::StateUpdatePacket, take_inbound_packet_body},
                outbound::{OutboundPacket, OutboundPacketBytes},
            },
            quirks::{TwoExtraEqBandSetEqualizerPacket, TwoExtraEqBandsValues},
            state::DeviceState,
            structures::{EqualizerConfiguration, PresetEqualizerProfile, STATE_UPDATE},
        },
    };

    struct A3933TestStateUpdatePacket {
        body: Vec<u8>,
    }
    impl OutboundPacket for A3933TestStateUpdatePacket {
        fn command(&self) -> [u8; 7] {
            STATE_UPDATE
        }

        fn body(&self) -> Vec<u8> {
            self.body.to_owned()
        }
    }

    #[test]
    fn it_remembers_eq_band_9_and_10_values() {
        let data = A3933TestStateUpdatePacket {
            body: vec![
                0x00, // host device
                0x00, // tws status
                0x00, 0x00, 0x00, 0x00, // dual battery
                b'0', b'0', b'.', b'0', b'0', // left firmware version
                b'0', b'0', b'.', b'0', b'0', // right firmware version
                b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
                b'0', b'0', // serial number
                0x00, 0x00, // eq profile id
                120, 120, 120, 120, 120, 120, 120, 120, 121, 122, // left eq
                120, 120, 120, 120, 120, 120, 120, 120, 123, 124,  // right eq
                0x00, // age range
                0x01, // hear id enabled
                120, 120, 120, 120, 120, 120, 120, 120, 125, 126, // left hear id
                120, 120, 120, 120, 120, 120, 120, 120, 127, 0, // right hear id
                0x00, 0x00, 0x00, 0x00, // hear id time
                0x00, // hear id type
                120, 120, 120, 120, 120, 120, 120, 120, 1, 2, // left hear id custom
                120, 120, 120, 120, 120, 120, 120, 120, 3, 4, // right hear id custom
                0x00, 0x00, // hear id eq profile
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, // custom button model
                0x07, // ambient sound mode cycle
                0x00, // ambient sound mode
                0x00, // noise canceling mode
                0x00, // transparency mode
                0x00, // custom noise canceling
                0xFF, 0xFF, // two unknown bytes
                0x00, // touch tone
                0x00, // wear detection
                0x00, // gaming mode
                0x00, // case battery
                0x00, // ?
                0x00, // device color
                0x00, // wind noise detection
                0xFF, 0xFF, 0xFF, // three unknown bytes
            ],
        }
        .bytes();

        let body = take_inbound_packet_body(&data).unwrap().1;
        let state_update = A3933StateUpdatePacket::take::<VerboseError<_>>(body)
            .unwrap()
            .1;
        let state: DeviceState = StateUpdatePacket::from(state_update).into();
        let dispatchers = A3933_DEVICE_PROFILE.custom_dispatchers.unwrap()();
        let state = (&dispatchers.packet_handlers()[&STATE_UPDATE])(&body, state);

        let equalizer_configuration =
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::TrebleReducer);
        let command_response = dispatchers
            .set_equalizer_configuration(state, equalizer_configuration.to_owned())
            .unwrap();

        assert_eq!(1, command_response.packets.len());
        assert_eq!(
            &TwoExtraEqBandSetEqualizerPacket {
                left_channel: &equalizer_configuration,
                right_channel: &equalizer_configuration,
                extra_band_values: TwoExtraEqBandsValues {
                    left_extra_1: 121,
                    left_extra_2: 122,
                    right_extra_1: 123,
                    right_extra_2: 124,
                },
            }
            .bytes(),
            command_response.packets.first().unwrap(),
        );
    }
}

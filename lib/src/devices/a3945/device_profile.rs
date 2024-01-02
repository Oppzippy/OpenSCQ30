use std::{
    collections::HashMap,
    sync::{
        atomic::{self, AtomicI32},
        Arc,
    },
};

use nom::error::VerboseError;

use crate::{
    device_profiles::DeviceProfile,
    devices::standard::{
        packets::outbound::{OutboundPacket, OutboundPacketBytes, SetEqualizerPacket},
        state::DeviceState,
        structures::{EqualizerConfiguration, STATE_UPDATE},
    },
    soundcore_device::device::{
        device_command_dispatcher::DeviceCommandDispatcher,
        packet_handlers::state_update::state_update_handler, soundcore_command::CommandResponse,
    },
};

use super::packets::take_a3945_state_update_packet;

pub const A3945_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
    sound_mode: None,
    has_hear_id: false,
    num_equalizer_channels: 2,
    num_equalizer_bands: 8,
    has_dynamic_range_compression: false,
    dynamic_range_compression_min_firmware_version: None,
    has_custom_button_model: true,
    has_wear_detection: false,
    has_touch_tone: false,
    has_auto_power_off: false,
    has_ambient_sound_mode_cycle: false,
    custom_dispatchers: Some(|| Arc::new(A3945Dispatcher::default())),
};

#[derive(Debug, Default)]
pub struct A3945Dispatcher {
    // The official app only displays 8 bands, so I have no idea what bands 9 and 10 do. We'll just keep track
    // of their initial value and resend that.
    band_9_and_10_left_and_right: Arc<AtomicI32>,
}

impl DeviceCommandDispatcher for A3945Dispatcher {
    fn packet_handlers(
        &self,
    ) -> HashMap<[u8; 7], Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
        let band_9_and_10 = self.band_9_and_10_left_and_right.to_owned();
        let mut handlers: HashMap<
            [u8; 7],
            Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>,
        > = HashMap::new();

        handlers.insert(
            STATE_UPDATE,
            Box::new(move |packet_bytes, state| {
                let result = take_a3945_state_update_packet::<VerboseError<_>>(packet_bytes);
                let packet = match result {
                    Ok((_, packet)) => packet,
                    Err(err) => {
                        tracing::error!("failed to parse packet: {err:?}");
                        return state;
                    }
                };
                let squashed_bytes = i32::from_ne_bytes([
                    packet.left_band_9_and_10[0],
                    packet.left_band_9_and_10[1],
                    packet.right_band_9_and_10[0],
                    packet.right_band_9_and_10[1],
                ]);
                band_9_and_10.store(squashed_bytes, atomic::Ordering::Relaxed);

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
        let band_9_and_10 = self
            .band_9_and_10_left_and_right
            .load(atomic::Ordering::Relaxed)
            .to_ne_bytes();

        let packet_bytes = CustomSetEqualizerPacket {
            left_channel,
            right_channel,
            left_band_9_and_10: [band_9_and_10[0], band_9_and_10[1]],
            right_band_9_and_10: [band_9_and_10[2], band_9_and_10[3]],
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

struct CustomSetEqualizerPacket<'a> {
    pub left_channel: &'a EqualizerConfiguration,
    pub right_channel: &'a EqualizerConfiguration,
    pub left_band_9_and_10: [u8; 2],
    pub right_band_9_and_10: [u8; 2],
}

impl<'a> OutboundPacket for CustomSetEqualizerPacket<'a> {
    fn command(&self) -> [u8; 7] {
        SetEqualizerPacket::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.left_channel
            .profile_id()
            .to_le_bytes()
            .into_iter()
            .chain(self.left_channel.volume_adjustments().bytes())
            .chain(self.left_band_9_and_10)
            .chain(self.right_channel.volume_adjustments().bytes())
            .chain(self.right_band_9_and_10)
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::{
        a3945::{
            device_profile::CustomSetEqualizerPacket, packets::take_a3945_state_update_packet,
        },
        standard::{
            packets::{
                inbound::{state_update_packet::StateUpdatePacket, take_inbound_packet_body},
                outbound::{OutboundPacket, OutboundPacketBytes},
            },
            state::DeviceState,
            structures::{EqualizerConfiguration, PresetEqualizerProfile, STATE_UPDATE},
        },
    };

    use super::A3945_DEVICE_PROFILE;

    struct A3945TestStateUpdatePacket {
        body: Vec<u8>,
    }
    impl OutboundPacket for A3945TestStateUpdatePacket {
        fn command(&self) -> [u8; 7] {
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
                0x00, // host device
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

        let body = take_inbound_packet_body(&data).unwrap().1;
        let state_update = take_a3945_state_update_packet::<VerboseError<_>>(body)
            .unwrap()
            .1;
        let state: DeviceState = StateUpdatePacket::from(state_update).into();
        let dispatchers = A3945_DEVICE_PROFILE.custom_dispatchers.unwrap()();
        let state = (&dispatchers.packet_handlers()[&STATE_UPDATE])(&body, state);

        let equalizer_configuration =
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::TrebleReducer);
        let command_response = dispatchers
            .set_equalizer_configuration(state, equalizer_configuration.to_owned())
            .unwrap();

        assert_eq!(1, command_response.packets.len());
        assert_eq!(
            &CustomSetEqualizerPacket {
                left_channel: &equalizer_configuration,
                right_channel: &equalizer_configuration,
                left_band_9_and_10: [121, 122],
                right_band_9_and_10: [123, 124],
            }
            .bytes(),
            command_response.packets.first().unwrap(),
        );
    }
}

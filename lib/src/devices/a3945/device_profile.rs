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
    custom_dispatchers: None,
};

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
            .chain(self.left_band_9_and_10.into_iter())
            .chain(self.right_channel.volume_adjustments().bytes())
            .chain(self.right_band_9_and_10.into_iter())
            .collect::<Vec<_>>()
    }
}

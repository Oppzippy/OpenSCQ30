use std::{collections::HashMap, sync::Arc};

use nom::error::VerboseError;

use crate::{
    device_profiles::DeviceProfile,
    devices::standard::{
        packets::{inbound::SoundModeTypeTwoUpdatePacket, outbound::OutboundPacketBytes},
        quirks::{TwoExtraEqBandSetEqualizerPacket, TwoExtraEqBands},
        state::DeviceState,
        structures::{EqualizerConfiguration, SOUND_MODE_UPDATE, STATE_UPDATE},
    },
    soundcore_device::device::{
        device_command_dispatcher::DeviceCommandDispatcher,
        packet_handlers::state_update::state_update_handler, soundcore_command::CommandResponse,
    },
};

use super::packets::A3936StateUpdatePacket;

pub const A3936_DEVICE_PROFILE: DeviceProfile = DeviceProfile {
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
    custom_dispatchers: Some(|| Arc::new(A3936Dispatcher::default())),
};

#[derive(Debug, Default)]
pub struct A3936Dispatcher {
    // The official app only displays 8 bands, so I have no idea what bands 9 and 10 do. We'll just keep track
    // of their initial value and resend that.
    extra_bands: Arc<TwoExtraEqBands>,
}

impl DeviceCommandDispatcher for A3936Dispatcher {
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
                let result = A3936StateUpdatePacket::take::<VerboseError<_>>(packet_bytes);
                let packet = match result {
                    Ok((_, packet)) => packet,
                    Err(err) => {
                        tracing::error!("failed to parse packet: {err:?}");
                        return state;
                    }
                };
                extra_bands.set_values(packet.extra_bands);

                // We only needed to capture information. The actual state transformation is passed on to the default handler..
                state_update_handler(packet_bytes, state)
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

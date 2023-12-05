use crate::{
    device_profiles::DeviceProfile,
    devices::standard::{
        packets::outbound::{OutboundPacket, OutboundPacketBytes, SetEqualizerPacket},
        state::DeviceState,
        structures::EqualizerConfiguration,
    },
    soundcore_device::device::{
        device_command_dispatcher::DeviceCommandDispatcher, soundcore_command::CommandResponse,
    },
};

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
    // The official app only displays 8 channels, so I have no idea what channel 9 and 10 do. We'll just keep track
    // of their initial value and resend that.
    left_channel_9_and_10: [u8; 2],
    right_channel_9_and_10: [u8; 2],
}

impl DeviceCommandDispatcher for A3945Dispatcher {
    fn set_equalizer_configuration(
        &self,
        state: DeviceState,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<CommandResponse> {
        let left_channel = &equalizer_configuration;
        let right_channel = &equalizer_configuration;

        let packet_bytes = CustomSetEqualizerPacket {
            left_channel,
            right_channel,
            left_channel_9_and_10: self.left_channel_9_and_10,
            right_channel_9_and_10: self.right_channel_9_and_10,
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
    pub left_channel_9_and_10: [u8; 2],
    pub right_channel_9_and_10: [u8; 2],
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
            .chain(self.left_channel_9_and_10.into_iter())
            .chain(self.right_channel.volume_adjustments().bytes())
            .chain(self.right_channel_9_and_10.into_iter())
            .collect::<Vec<_>>()
    }
}

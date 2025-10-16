use std::collections::HashMap;

use crate::devices::soundcore::{
    a3945::{packets::A3945StateUpdatePacket, state::A3945State},
    common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        packet::outbound::{OutboundPacketBytesExt, RequestState},
    },
};

mod packets;
mod state;

soundcore_device!(
    A3945State,
    A3945StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3945State, A3945StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.equalizer().await;
        builder.button_configuration();
        builder.touch_tone();
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3945StateUpdatePacket::default().bytes(),
        )])
    },
);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        api::settings::{SettingId, Value},
        devices::{
            DeviceModel,
            soundcore::common::{
                device::test_utils::TestSoundcoreDevice,
                packet::{self, Direction, Packet},
                structures::{EqualizerConfiguration, PresetEqualizerProfile},
            },
        },
    };

    #[tokio::test(start_paused = true)]
    async fn it_remembers_band_9_and_10_values() {
        let state_update_packet = Packet {
            direction: Direction::Inbound,
            command: packet::inbound::STATE_COMMAND,
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
        };

        let mut device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3945,
            HashMap::from([(packet::inbound::STATE_COMMAND, state_update_packet)]),
        )
        .await;

        device
            .assert_set_settings_response(
                vec![(
                    SettingId::PresetEqualizerProfile,
                    Value::OptionalString(Some("TrebleReducer".into())),
                )],
                vec![
                    packet::outbound::SetEqualizer {
                        equalizer_configuration:
                            &EqualizerConfiguration::<2, 10>::new_from_preset_profile(
                                PresetEqualizerProfile::TrebleReducer,
                                [vec![1, 2], vec![3, 4]],
                            ),
                    }
                    .into(),
                ],
            )
            .await;
    }
}

use std::collections::HashMap;

pub use crate::devices::soundcore::common::modules::button_configuration_v2::COMMON_SETTINGS as BUTTON_CONFIGURATION_SETTINGS;
use crate::devices::soundcore::{
    a3933::{packets::inbound::A3933StateUpdatePacket, state::A3933State},
    common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        modules::sound_modes::AvailableSoundModes,
        packet::outbound::{OutboundPacketBytesExt, RequestState},
        structures::{AmbientSoundMode, NoiseCancelingMode, TransparencyMode},
    },
};

mod packets;
mod state;

soundcore_device!(
    A3933State,
    A3933StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3933State, A3933StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.sound_modes(AvailableSoundModes {
            ambient_sound_modes: vec![
                AmbientSoundMode::Normal,
                AmbientSoundMode::Transparency,
                AmbientSoundMode::NoiseCanceling,
            ],
            transparency_modes: vec![
                TransparencyMode::FullyTransparent,
                TransparencyMode::VocalMode,
            ],
            noise_canceling_modes: vec![
                NoiseCancelingMode::Transport,
                NoiseCancelingMode::Indoor,
                NoiseCancelingMode::Outdoor,
            ],
        });
        builder.equalizer().await;
        builder.button_configuration_v2(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();
        builder.touch_tone();
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3933StateUpdatePacket::default().bytes(),
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

    use super::*;

    #[tokio::test(start_paused = true)]
    async fn it_remembers_eq_band_9_and_10_values() {
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
        };

        let mut device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3933,
            HashMap::from([(RequestState::COMMAND, state_update_packet)]),
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

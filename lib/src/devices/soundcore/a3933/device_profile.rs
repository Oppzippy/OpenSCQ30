use std::collections::HashMap;

use crate::devices::soundcore::standard::{
    device::fetch_state_from_state_update_packet,
    macros::soundcore_device,
    modules::sound_modes::AvailableSoundModes,
    packets::outbound::{OutboundPacketBytesExt, RequestStatePacket},
    structures::{AmbientSoundMode, NoiseCancelingMode, TransparencyMode},
};

use super::{packets::inbound::A3933StateUpdatePacket, state::A3933State};

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
        builder.button_configuration();
        builder.ambient_sound_mode_cycle();
        builder.tws_status();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestStatePacket::COMMAND,
            A3933StateUpdatePacket::default().bytes(),
        )])
    },
);

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use macaddr::MacAddr6;
    use tokio::sync::mpsc;

    use crate::{
        api::{
            device::OpenSCQ30DeviceRegistry,
            settings::{SettingId, Value},
        },
        connection_backend::rfcomm::MockRfcommBackend,
        devices::{
            DeviceModel,
            soundcore::standard::{
                packets::{
                    Packet,
                    inbound::state_update_packet,
                    outbound::{OutboundPacket, OutboundPacketBytesExt, set_equalizer},
                },
                structures::{Command, EqualizerConfiguration, PresetEqualizerProfile},
            },
        },
        storage::OpenSCQ30Database,
    };

    struct A3933TestStateUpdatePacket {
        body: Vec<u8>,
    }
    impl OutboundPacket for A3933TestStateUpdatePacket {
        fn command(&self) -> Command {
            state_update_packet::COMMAND
        }

        fn body(&self) -> Vec<u8> {
            self.body.to_owned()
        }
    }

    #[tokio::test]
    async fn it_remembers_eq_band_9_and_10_values() {
        let data = A3933TestStateUpdatePacket {
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
        }
        .bytes();

        let (inbound_sender, inbound_receiver) = mpsc::channel(10);
        let (outbound_sender, mut outbound_receiver) = mpsc::channel(10);
        let database = Arc::new(OpenSCQ30Database::new_in_memory().await.unwrap());
        let registry = super::device_registry(
            MockRfcommBackend::new(inbound_receiver, outbound_sender),
            database,
            DeviceModel::SoundcoreA3933,
        );
        inbound_sender.send(data).await.unwrap();
        let device = registry.connect(MacAddr6::nil()).await.unwrap();
        _ = outbound_receiver
            .recv()
            .await
            .expect("state update packet request");

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(1)).await;
            inbound_sender
                .send(
                    Packet {
                        command: set_equalizer::COMMAND.to_inbound(),
                        body: Vec::new(),
                    }
                    .bytes(),
                )
                .await
                .unwrap();
        });
        device
            .set_setting_values(vec![(
                SettingId::PresetProfile,
                Value::OptionalString(Some("TrebleReducer".into())),
            )])
            .await
            .unwrap();

        let set_eq_packet_bytes = outbound_receiver.recv().await.unwrap();
        assert_eq!(
            Packet::from(set_equalizer::SetEqualizerPacket {
                equalizer_configuration: &EqualizerConfiguration::<2, 10>::new_from_preset_profile(
                    PresetEqualizerProfile::TrebleReducer,
                    [vec![1, 2], vec![3, 4]],
                ),
            })
            .bytes(),
            set_eq_packet_bytes
        );
    }
}

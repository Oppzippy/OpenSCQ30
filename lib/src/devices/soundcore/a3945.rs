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
    use std::{sync::Arc, time::Duration};

    use macaddr::MacAddr6;
    use tokio::sync::mpsc;

    use crate::{
        api::{
            device::OpenSCQ30DeviceRegistry,
            settings::{SettingId, Value},
        },
        connection_backend::mock::rfcomm::MockRfcommBackend,
        devices::{
            DeviceModel,
            soundcore::common::{
                packet::{
                    self, Command, Direction, Packet,
                    outbound::{OutboundPacket, OutboundPacketBytesExt},
                },
                structures::{EqualizerConfiguration, PresetEqualizerProfile},
            },
        },
        storage::OpenSCQ30Database,
    };

    struct A3945TestStateUpdatePacket {
        body: Vec<u8>,
    }
    impl OutboundPacket for A3945TestStateUpdatePacket {
        fn command(&self) -> Command {
            packet::inbound::STATE_COMMAND
        }

        fn body(&self) -> Vec<u8> {
            self.body.to_owned()
        }
    }

    #[tokio::test]
    async fn it_remembers_band_9_and_10_values() {
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

        let (inbound_sender, inbound_receiver) = mpsc::channel(10);
        let (outbound_sender, mut outbound_receiver) = mpsc::channel(10);
        let database = Arc::new(OpenSCQ30Database::new_in_memory().await.unwrap());
        let registry = super::device_registry(
            MockRfcommBackend::new(inbound_receiver, outbound_sender),
            database,
            DeviceModel::SoundcoreA3945,
        );
        inbound_sender.send(data).await.unwrap();
        let device = registry.connect(MacAddr6::nil()).await.unwrap();
        _ = outbound_receiver
            .recv()
            .await
            .expect("state update packet request");

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            inbound_sender
                .send(
                    Packet {
                        direction: Direction::Inbound,
                        command: packet::outbound::SET_EQUALIZER_COMMAND,
                        body: Vec::new(),
                    }
                    .bytes(),
                )
                .await
                .unwrap();
        });
        device
            .set_setting_values(vec![(
                SettingId::PresetEqualizerProfile,
                Value::OptionalString(Some("TrebleReducer".into())),
            )])
            .await
            .unwrap();

        let set_eq_packet_bytes = outbound_receiver.recv().await.unwrap();
        assert_eq!(
            Packet::from(packet::outbound::SetEqualizer {
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

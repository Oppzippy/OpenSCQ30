use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3028::{packets::A3028StateUpdatePacket, state::A3028State},
        common::{
            device::fetch_state_from_state_update_packet,
            macros::soundcore_device,
            modules::sound_modes::AvailableSoundModes,
            packet::outbound::{RequestState, ToPacket},
            structures::{AmbientSoundMode, NoiseCancelingMode},
        },
    },
    i18n::fl,
};

mod packets;
mod state;

soundcore_device!(
    A3028State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3028State, A3028StateUpdatePacket>(packet_io)
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
            transparency_modes: vec![],
            noise_canceling_modes: vec![
                NoiseCancelingMode::Transport,
                NoiseCancelingMode::Indoor,
                NoiseCancelingMode::Outdoor,
            ],
        });
        builder.equalizer().await;
        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);
        builder.single_battery();
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3028StateUpdatePacket::default().to_packet().bytes(),
        )])
    },
);

#[derive(IntoStaticStr, VariantArray)]
#[allow(clippy::enum_variant_names)]
enum AutoPowerOffDuration {
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[strum(serialize = "60m")]
    SixtyMinutes,
    #[strum(serialize = "90m")]
    NinetyMinutes,
    #[strum(serialize = "120m")]
    OneHundredTwentyMinutes,
}

impl Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            Self::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            Self::SixtyMinutes => fl!("x-minutes", minutes = 60),
            Self::NinetyMinutes => fl!("x-minutes", minutes = 90),
            Self::OneHundredTwentyMinutes => fl!("x-minutes", minutes = 120),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, sync::Arc, time::Duration};

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
                packet,
                structures::{AmbientSoundMode, NoiseCancelingMode},
            },
        },
        storage::OpenSCQ30Database,
    };

    fn example_state_update_packet() -> Vec<u8> {
        vec![
            0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01, 0x46, 0x00, 0x05, 0x00, 0xfe, 0xfe, 0x3c,
            0xb4, 0x8f, 0xa0, 0x8e, 0xb4, 0x74, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x30, 0x32, 0x2e, 0x33, 0x30, 0x33, 0x30, 0x32,
            0x39, 0x30, 0x38, 0x36, 0x45, 0x43, 0x38, 0x32, 0x46, 0x31, 0x32, 0x41, 0x43, 0x30,
        ]
    }

    async fn create_test_connection() -> (
        impl OpenSCQ30DeviceRegistry,
        mpsc::Sender<Vec<u8>>,
        mpsc::Receiver<Vec<u8>>,
    ) {
        let (inbound_sender, inbound_receiver) = mpsc::channel(10);
        let (outbound_sender, outbound_receiver) = mpsc::channel(10);
        let database = Arc::new(OpenSCQ30Database::new_in_memory().await.unwrap());
        let registry = super::device_registry(
            MockRfcommBackend::new(inbound_receiver, outbound_sender),
            database,
            DeviceModel::SoundcoreA3028,
        );
        (registry, inbound_sender, outbound_receiver)
    }

    #[tokio::test(start_paused = true)]
    async fn test_new_with_example_state_update_packet() {
        let (registry, inbound_sender, mut outbound_receiver) = create_test_connection().await;
        inbound_sender
            .send(example_state_update_packet())
            .await
            .unwrap();
        let device = registry.connect(MacAddr6::nil()).await.unwrap();
        _ = outbound_receiver
            .recv()
            .await
            .expect("state update packet request");
        assert_eq!(
            AmbientSoundMode::Normal,
            Value::from(device.setting(&SettingId::AmbientSoundMode).unwrap())
                .try_as_enum_variant()
                .unwrap()
        );
        assert_eq!(
            NoiseCancelingMode::Transport,
            Value::from(device.setting(&SettingId::NoiseCancelingMode).unwrap())
                .try_as_enum_variant()
                .unwrap()
        );
        assert_eq!(
            None,
            Value::from(device.setting(&SettingId::PresetEqualizerProfile).unwrap())
                .try_as_optional_str()
                .unwrap()
        );
        assert_eq!(
            vec![-60, 60, 23, 40, 22, 60, -4, 16],
            Value::from(device.setting(&SettingId::VolumeAdjustments).unwrap())
                .try_into_i16_vec()
                .unwrap()
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_new_with_retry() {
        let (registry, inbound_sender, mut outbound_receiver) = create_test_connection().await;
        tokio::spawn(async move {
            outbound_receiver
                .recv()
                .await
                .expect("state update packet request");
            tokio::time::sleep(Duration::from_millis(1500)).await;
            inbound_sender
                .send(example_state_update_packet())
                .await
                .unwrap();
        });
        registry
            .connect(MacAddr6::nil())
            .await
            .expect("should not time out");
    }

    #[tokio::test(start_paused = true)]
    async fn test_new_max_retries() {
        let (registry, _inbound_sender, _outbound_receiver) = create_test_connection().await;
        registry
            .connect(MacAddr6::nil())
            .await
            .err()
            .expect("should time out");
    }

    #[tokio::test(start_paused = true)]
    async fn test_ambient_sound_mode_update_packet() {
        let (registry, inbound_sender, mut _outbound_receiver) = create_test_connection().await;
        inbound_sender
            .send(example_state_update_packet())
            .await
            .unwrap();
        let device = registry.connect(MacAddr6::nil()).await.unwrap();

        // alert from device that sound mode changed
        inbound_sender
            .send(vec![
                0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0e, 0x00, 0x00, 0x01, 0x01, 0x00, 0x20,
            ])
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(1)).await;

        assert_eq!(
            AmbientSoundMode::NoiseCanceling,
            Value::from(device.setting(&SettingId::AmbientSoundMode).unwrap())
                .try_as_enum_variant()
                .unwrap()
        );
        assert_eq!(
            NoiseCancelingMode::Outdoor,
            Value::from(device.setting(&SettingId::NoiseCancelingMode).unwrap())
                .try_as_enum_variant()
                .unwrap()
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_set_sound_mode_doesnt_resend_if_nothing_changed() {
        let (registry, inbound_sender, mut outbound_receiver) = create_test_connection().await;
        inbound_sender
            .send(example_state_update_packet())
            .await
            .unwrap();
        let device = registry.connect(MacAddr6::nil()).await.unwrap();
        outbound_receiver
            .recv()
            .await
            .expect("state update packet request");

        let f = tokio::spawn(async move {
            outbound_receiver.recv().await.expect("sound mode update");
            inbound_sender
                .send(
                    packet::outbound::SetSoundModes::COMMAND
                        .ack::<packet::InboundMarker>()
                        .bytes(),
                )
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
            if !outbound_receiver.is_empty() {
                let packet = outbound_receiver.recv().await.unwrap();
                panic!(
                    "another packet should not be sent since nothing has changed, but got {packet:?}"
                );
            }
        });

        // Changed sound mode
        device
            .set_setting_values(vec![(
                SettingId::AmbientSoundMode,
                Cow::from("Transparency").into(),
            )])
            .await
            .unwrap();
        // Unchanged sound mode
        device
            .set_setting_values(vec![(
                SettingId::AmbientSoundMode,
                Cow::from("Transparency").into(),
            )])
            .await
            .unwrap();
        f.await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn test_set_equalizer_configuration_called_twice() {
        let (registry, inbound_sender, mut outbound_receiver) = create_test_connection().await;
        inbound_sender
            .send(example_state_update_packet())
            .await
            .unwrap();
        let device = registry.connect(MacAddr6::nil()).await.unwrap();
        outbound_receiver
            .recv()
            .await
            .expect("state update packet request");

        let f = tokio::spawn(async move {
            outbound_receiver.recv().await.expect("equalizer update");
            inbound_sender
                .send(
                    packet::outbound::SET_EQUALIZER_COMMAND
                        .ack::<packet::InboundMarker>()
                        .bytes(),
                )
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
            if !outbound_receiver.is_empty() {
                let packet = outbound_receiver.recv().await.unwrap();
                panic!(
                    "another packet should not be sent since nothing has changed, but got {packet:?}"
                );
            }
        });
        // Changed equalizer
        device
            .set_setting_values(vec![(
                SettingId::PresetEqualizerProfile,
                Cow::from("Acoustic").into(),
            )])
            .await
            .unwrap();

        // Unchanged equaalizer
        device
            .set_setting_values(vec![(
                SettingId::PresetEqualizerProfile,
                Cow::from("Acoustic").into(),
            )])
            .await
            .unwrap();
        f.await.unwrap();
    }
}

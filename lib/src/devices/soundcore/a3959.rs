use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        packet::outbound::{OutboundPacketBytesExt, RequestState},
    },
    i18n::fl,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    state::A3959State,
    packets::inbound::A3959State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, state::A3959State, packets::inbound::A3959State>(
            packet_io,
        )
        .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3959_sound_modes();
        builder.equalizer().await;
        builder.a3959_button_configuration();
        builder.ambient_sound_mode_cycle();
        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);
        builder.touch_tone();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            packets::inbound::A3959State::default().bytes(),
        )])
    },
);

#[derive(IntoStaticStr, VariantArray)]
#[allow(clippy::enum_variant_names)]
enum AutoPowerOffDuration {
    #[strum(serialize = "10m")]
    TenMinutes,
    #[strum(serialize = "20m")]
    TwentyMinutes,
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[strum(serialize = "60m")]
    SixtyMinutes,
}

impl Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            Self::TenMinutes => fl!("x-minutes", minutes = 10),
            Self::TwentyMinutes => fl!("x-minutes", minutes = 20),
            Self::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            Self::SixtyMinutes => fl!("x-minutes", minutes = 60),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use macaddr::MacAddr6;
    use tokio::sync::mpsc;

    use crate::{
        DeviceModel,
        device::OpenSCQ30DeviceRegistry,
        devices::soundcore::common::packet::{Command, Direction, Packet},
        mock::rfcomm::MockRfcommBackend,
        settings::{SettingId, Value},
        storage::OpenSCQ30Database,
    };

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
            DeviceModel::SoundcoreA3959,
        );
        (registry, inbound_sender, outbound_receiver)
    }

    #[tokio::test(start_paused = true)]
    async fn test_with_packet_from_github_issue_149() {
        let (registry, inbound_sender, mut outbound_receiver) = create_test_connection().await;
        inbound_sender
            .send(
                Packet {
                    direction: Direction::Inbound,
                    command: Command([1, 1]),
                    body: vec![
                        1, 1, 5, 6, 255, 255, 48, 49, 46, 54, 52, 48, 49, 46, 54, 52, 51, 57, 53,
                        57, 68, 69, 68, 54, 54, 57, 50, 68, 66, 54, 70, 52, 254, 254, 101, 120,
                        161, 171, 171, 152, 144, 179, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
                        241, 240, 102, 102, 242, 243, 68, 68, 51, 0, 85, 0, 0, 1, 255, 1, 49, 1, 1,
                        0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                }
                .bytes(),
            )
            .await
            .unwrap();
        let device = registry
            .connect(MacAddr6::nil())
            .await
            .expect("should parse packet");
        _ = outbound_receiver
            .recv()
            .await
            .expect("state update packet request");

        assert_eq!(
            Value::from(device.setting(&SettingId::AmbientSoundMode).unwrap())
                .try_as_str()
                .unwrap(),
            "NoiseCanceling"
        );
        assert_eq!(
            Value::from(device.setting(&SettingId::WindNoiseSuppression).unwrap())
                .try_as_bool()
                .unwrap(),
            true,
        );

        let expected_button_actions = [
            (SettingId::LeftSinglePress, Some("VolumeDown")),
            (SettingId::LeftDoublePress, Some("PlayPause")),
            (SettingId::LeftTriplePress, Some("PreviousSong")),
            (SettingId::LeftLongPress, Some("AmbientSoundMode")),
            (SettingId::RightSinglePress, Some("VolumeUp")),
            (SettingId::RightDoublePress, Some("PlayPause")),
            (SettingId::RightTriplePress, Some("NextSong")),
            (SettingId::RightLongPress, Some("AmbientSoundMode")),
        ];
        for (setting_id, expected_action) in expected_button_actions {
            assert_eq!(
                Value::from(device.setting(&setting_id).unwrap())
                    .try_as_optional_str()
                    .unwrap(),
                expected_action,
                "{setting_id} should be {expected_action:?}"
            );
        }
    }

    #[tokio::test(start_paused = true)]
    async fn test_with_packet_from_github_issue_149_modified_to_disable_tws() {
        let (registry, inbound_sender, mut outbound_receiver) = create_test_connection().await;
        inbound_sender
            .send(
                Packet {
                    direction: Direction::Inbound,
                    command: Command([1, 1]),
                    body: vec![
                        1, 0, 5, 6, 255, 255, 48, 49, 46, 54, 52, 48, 49, 46, 54, 52, 51, 57, 53,
                        57, 68, 69, 68, 54, 54, 57, 50, 68, 66, 54, 70, 52, 254, 254, 101, 120,
                        161, 171, 171, 152, 144, 179, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
                        241, 240, 102, 102, 242, 243, 68, 68, 51, 0, 85, 0, 0, 1, 255, 1, 49, 1, 1,
                        0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                }
                .bytes(),
            )
            .await
            .unwrap();
        let device = registry
            .connect(MacAddr6::nil())
            .await
            .expect("should parse packet");
        _ = outbound_receiver
            .recv()
            .await
            .expect("state update packet request");

        let expectation = [
            (SettingId::LeftSinglePress, None),
            (SettingId::LeftDoublePress, Some("PlayPause")),
            (SettingId::LeftTriplePress, None),
            (SettingId::LeftLongPress, Some("AmbientSoundMode")),
            (SettingId::RightSinglePress, None),
            (SettingId::RightDoublePress, Some("PlayPause")),
            (SettingId::RightTriplePress, None),
            (SettingId::RightLongPress, Some("AmbientSoundMode")),
        ];
        for (setting_id, expected_value) in expectation {
            assert_eq!(
                Value::from(device.setting(&setting_id).unwrap(),)
                    .try_as_optional_str()
                    .unwrap(),
                expected_value,
                "{setting_id} should be {expected_value:?}"
            );
        }
    }

    #[tokio::test(start_paused = true)]
    async fn test_with_other_packet_from_github_issue_149() {
        let (registry, inbound_sender, mut outbound_receiver) = create_test_connection().await;
        inbound_sender
            .send(
                Packet {
                    direction: Direction::Inbound,
                    command: Command([1, 1]),
                    body: vec![
                        0, 1, 6, 6, 255, 255, 48, 49, 46, 54, 53, 48, 49, 46, 54, 53, 51, 57, 53,
                        57, 57, 48, 49, 66, 69, 55, 50, 67, 57, 67, 49, 56, 14, 0, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 255,
                        255, 99, 102, 255, 255, 68, 68, 55, 0, 85, 0, 0, 1, 255, 1, 49, 1, 1, 1, 1,
                        2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                }
                .bytes(),
            )
            .await
            .unwrap();
        let _device = registry
            .connect(MacAddr6::nil())
            .await
            .expect("should parse packet");
        _ = outbound_receiver
            .recv()
            .await
            .expect("state update packet request");
    }
}

use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        modules::button_configuration_v2::{
            ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings, COMMON_TWS_ACTIONS,
        },
        packet::outbound::{OutboundPacketBytesExt, RequestState},
        structures::button_configuration_v2::{
            ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
        },
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
        builder.equalizer_with_drc().await;
        builder.button_configuration_v2(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();
        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);
        builder.touch_tone();
        builder.dual_battery(10);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            packets::inbound::A3959State::default().bytes(),
        )])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<8, 4> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        order: [
            Button::LeftSinglePress,
            Button::RightSinglePress,
            Button::LeftDoublePress,
            Button::RightDoublePress,
            Button::LeftTriplePress,
            Button::RightTriplePress,
            Button::LeftLongPress,
            Button::RightLongPress,
        ],
        settings: [
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_TWS_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_TWS_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 5,
                press_kind: ButtonPressKind::Triple,
                available_actions: COMMON_TWS_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_TWS_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
        ],
    };

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
    use std::{collections::BTreeSet, sync::Arc, time::Duration};

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

        let expected_values: [(SettingId, Value); _] = [
            (SettingId::BatteryLevelLeft, "5".into()),
            (SettingId::BatteryLevelRight, "6".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::AmbientSoundMode, "NoiseCanceling".into()),
            (SettingId::AdaptiveNoiseCanceling, "5/5".into()),
            (SettingId::ManualNoiseCanceling, 5.into()),
            (SettingId::MultiSceneNoiseCanceling, "Outdoor".into()),
            (SettingId::WindNoiseSuppression, true.into()),
            (SettingId::LeftSinglePress, Some("VolumeDown").into()),
            (SettingId::LeftDoublePress, Some("PlayPause").into()),
            (SettingId::LeftTriplePress, Some("PreviousSong").into()),
            (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            (SettingId::RightSinglePress, Some("VolumeUp").into()),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (SettingId::RightTriplePress, Some("NextSong").into()),
            (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
            (SettingId::TouchTone, true.into()),
            (SettingId::AutoPowerOff, "10m".into()),
        ];
        for (setting_id, expected) in expected_values {
            let setting = device
                .setting(&setting_id)
                .expect(&format!("{setting_id} returned None"));
            assert_eq!(
                Value::from(setting),
                expected,
                "{setting_id} should be {expected:?}"
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
            (SettingId::LeftSinglePress, Value::OptionalString(None)),
            (SettingId::LeftDoublePress, Some("PlayPause").into()),
            (SettingId::LeftTriplePress, Value::OptionalString(None)),
            (SettingId::LeftLongPress, Some("AmbientSoundMode").into()),
            (SettingId::RightSinglePress, Value::OptionalString(None)),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (SettingId::RightTriplePress, Value::OptionalString(None)),
            (SettingId::RightLongPress, Some("AmbientSoundMode").into()),
        ];
        for (setting_id, expected_value) in expectation {
            let setting = device
                .setting(&setting_id)
                .expect(&format!("{setting_id} returned None"));
            assert_eq!(
                Value::from(setting),
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

    #[tokio::test(start_paused = true)]
    async fn test_set_multiple_button_actions() {
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

        tokio::spawn(async move {
            let ok_packet = Packet {
                direction: Direction::Inbound,
                command: Command([0x04, 0x81]),
                body: Vec::new(),
            }
            .bytes();
            tokio::time::sleep(Duration::from_millis(50)).await;
            inbound_sender.send(ok_packet.clone()).await.unwrap();
            tokio::time::sleep(Duration::from_millis(50)).await;
            inbound_sender.send(ok_packet).await.unwrap();
        });
        device
            .set_setting_values(vec![
                (SettingId::LeftSinglePress, "VolumeUp".into()),
                (SettingId::RightSinglePress, "VolumeDown".into()),
            ])
            .await
            .unwrap();

        let packets = BTreeSet::from_iter([
            outbound_receiver.recv().await.expect("first button action"),
            outbound_receiver
                .recv()
                .await
                .expect("second button action"),
        ]);

        let expected_packets = BTreeSet::from_iter([
            // Left Volume Up
            Packet {
                direction: Direction::Outbound,
                command: Command([0x04, 0x81]),
                body: vec![0, 2, 0xF0],
            }
            .bytes(),
            // Right Volume Down
            Packet {
                direction: Direction::Outbound,
                command: Command([0x04, 0x81]),
                body: vec![1, 2, 0xF1],
            }
            .bytes(),
        ]);

        assert_eq!(packets, expected_packets);
    }

    #[tokio::test(start_paused = true)]
    async fn test_set_equalizer_configuration() {
        let (registry, inbound_sender, mut outbound_receiver) = create_test_connection().await;
        inbound_sender
            .send(
                Packet {
                    direction: Direction::Inbound,
                    command: Command([1, 1]),
                    body: vec![
                        1, 1, 0, 9, 255, 255, 48, 49, 46, 54, 52, 48, 49, 46, 54, 52, 51, 57, 53,
                        57, 68, 69, 68, 54, 54, 57, 50, 68, 66, 54, 70, 52, 254, 254, 101, 120,
                        161, 171, 171, 152, 144, 60, 120, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
                        241, 240, 102, 102, 242, 243, 68, 68, 51, 0, 0x55, 0, 0, 1, 255, 1, 49, 1,
                        1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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

        tokio::spawn(async move {
            let ok_packet = Packet {
                direction: Direction::Inbound,
                command: Command([0x02, 0x83]),
                body: Vec::new(),
            }
            .bytes();
            tokio::time::sleep(Duration::from_millis(50)).await;
            inbound_sender.send(ok_packet.clone()).await.unwrap();
        });
        device
            .set_setting_values(vec![(
                SettingId::VolumeAdjustments,
                Value::I16Vec(vec![-19, 0, 41, 51, 51, 32, 13, -35]),
            )])
            .await
            .unwrap();

        let packet = outbound_receiver
            .recv()
            .await
            .expect("set equalizer configuration");

        assert_eq!(
            packet,
            vec![
                8, 238, 0, 0, 0, 2, 131, 32, 0, 254, 254, 101, 120, 161, 171, 171, 152, 133, 85,
                120, 120, 118, 119, 124, 123, 124, 121, 123, 114, 120, 120, 131
            ]
        );
    }
}

use std::collections::HashMap;

use crate::{
    devices::soundcore::{
        common::{
            self,
            macros::soundcore_device,
            modules::{
                button_configuration::{
                    ButtonAction, ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                },
                dual_battery_level::DualBatteryLevelConfiguration,
            },
            packet::{
                inbound::{DualConnectionsDevicePacket, TryToPacket},
                outbound::{RequestState, ToPacket},
            },
            structures::button_configuration::{
                ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
            },
        },
        d1301::{packets::inbound::D1301StateUpdatePacket, state::D1301State},
    },
    i18n::fl,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    D1301State,
    async |packet_io| {
        let state_update_packet: packets::inbound::D1301StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let auto_stop_timer_packet: packets::inbound::AutoStopTimerPacket = packet_io
            .send_with_response(&packets::outbound::request_auto_stop_timer())
            .await?
            .try_to_packet()?;
        let alarms_packet: packets::inbound::AlarmsPacket = packet_io
            .send_with_response(&packets::outbound::request_alarms())
            .await?
            .try_to_packet()?;
        // Unlike other models, the D1301 answers this even while dual connections is disabled, and the
        // state update packet has no enabled flag, so always ask.
        let dual_connections_devices =
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?;
        Ok(state::D1301State::new(
            state_update_packet,
            auto_stop_timer_packet.0,
            alarms_packet.0,
            modules::dual_connections::initial_dual_connections(dual_connections_devices),
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.d1301_tap_controls_disabled();
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.reset_button_configuration::<packets::inbound::D1301StateUpdatePacket>(
            RequestState.to_packet(),
        );

        builder.d1301_alarms();

        builder.d1301_dual_connections().await;

        // misc
        builder.d1301_auto_stop_timer();
        builder.d1301_auto_switch_once_asleep();
        builder.d1301_listening_mode();
        builder.d1301_noise_canceling();

        // misc: prompts
        builder.d1301_incoming_calls_during_bluetooth_mode();
        builder.low_battery_prompt();
        builder.d1301_auto_power_off_prompt();
        builder.d1301_listening_mode_prompt();
        builder.d1301_noise_canceling_prompt();

        // info
        builder.tws_status();
        builder.dual_battery_level_custom(DualBatteryLevelConfiguration {
            max_level: 10,
            level_offset: 1,
        });
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestState::COMMAND,
                D1301StateUpdatePacket::default().to_packet(),
            ),
            (
                packets::inbound::AutoStopTimerPacket::COMMAND,
                packets::inbound::AutoStopTimerPacket(structures::AutoStopTimer::default())
                    .to_packet(),
            ),
            (
                packets::inbound::AlarmsPacket::COMMAND,
                packets::inbound::AlarmsPacket(Vec::new()).to_packet(),
            ),
            (
                DualConnectionsDevicePacket::COMMAND,
                DualConnectionsDevicePacket::demo().to_packet(),
            ),
        ])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<4, 2> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: false,
        set_button_action_command_override: None,
        setting_id_override: None,
        order: [
            Button::LeftDoublePress,
            Button::RightDoublePress,
            Button::LeftTriplePress,
            Button::RightTriplePress,
        ],
        settings: [
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: BUTTON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::None,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 5,
                press_kind: ButtonPressKind::Triple,
                available_actions: BUTTON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
        ],
    };

const BUTTON_ACTIONS: &[ButtonAction] = &[
    ButtonAction {
        id: 0,
        name: "VolumeUp",
        localized_name: || fl!("volume-up"),
    },
    ButtonAction {
        id: 1,
        name: "VolumeDown",
        localized_name: || fl!("volume-down"),
    },
    ButtonAction {
        id: 2,
        name: "PreviousSong",
        localized_name: || fl!("previous-song"),
    },
    ButtonAction {
        id: 3,
        name: "NextSong",
        localized_name: || fl!("next-song"),
    },
    ButtonAction {
        id: 6,
        name: "PlayPause",
        localized_name: || fl!("play-pause"),
    },
    ButtonAction {
        id: 8,
        name: "NoiseCanceling",
        localized_name: || fl!("noise-canceling"),
    },
    ButtonAction {
        id: 13,
        name: "ChangeMode",
        localized_name: || fl!("change-mode"),
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
            packet,
        },
        settings::SettingId,
    };

    #[tokio::test(start_paused = true)]
    async fn settings_match_soundcore_app() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1301,
            HashMap::from([
                (
                    packet::Command([1, 1]),
                    packet::Inbound::new(
                        packet::Command([1, 1]),
                        vec![
                            1, 1, 9, 9, 56, 49, 46, 48, 48, 56, 49, 46, 48, 48, 49, 51, 48, 49, 48,
                            48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 223, 186, 112, 217, 191,
                            144, 48, 49, 46, 48, 48, 0, 0, 0, 0, 6, 0xdd, 0x66, 0x00, 0x00, 255,
                            255, 255, 255, 255, 255, 255, 255, 0, 30, 0, 1, 255, 255, 255, 0, 0, 0,
                            0, 0, 50, 17, 1, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 203, 49, 61, 0,
                            0, 0, 0, 0, 2, 2, 1, 128, 0, 0, 0, 0, 0, 0, 4, 3, 2, 128, 0, 0, 0, 0,
                            0, 0, 4, 4, 3, 128, 0, 0, 0, 0, 0, 0, 4, 5, 4, 128, 0, 0, 0, 0, 0, 0,
                            4, 1, 0, 0, 1, 8, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0,
                        ],
                    ),
                ),
                (
                    packets::inbound::AutoStopTimerPacket::COMMAND,
                    packet::Inbound::new(
                        packets::inbound::AutoStopTimerPacket::COMMAND,
                        vec![0, 30, 0, 1, 60, 0, 0, 0],
                    ),
                ),
                (
                    packets::inbound::AlarmsPacket::COMMAND,
                    packet::Inbound::new(
                        packets::inbound::AlarmsPacket::COMMAND,
                        vec![
                            0, 0, 149, 1, 0b00000000, 3, 50, 10, 1, 1, 149, 2, 0b10000000, 0, 75, 5,
                        ],
                    ),
                ),
            ]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::BatteryLevelLeft, "10/10".into()),
            (SettingId::BatteryLevelRight, "10/10".into()),
            (SettingId::NoiseCanceling, false.into()),
            (SettingId::ListeningMode, "Bluetooth".into()),
            (SettingId::AutoStopTimer, false.into()),
            (SettingId::AutoStopTimerDuration, 30.into()),
            (SettingId::ButtonsEnabled, true.into()),
            (SettingId::LeftDoublePress, Some("ChangeMode").into()),
            (SettingId::LeftTriplePress, Some("VolumeUp").into()),
            (SettingId::RightDoublePress, Some("PlayPause").into()),
            (SettingId::RightTriplePress, Some("VolumeUp").into()),
            (SettingId::DefaultListeningMode, "Bluetooth".into()),
            (SettingId::IncomingCallsDuringBluetoothMode, false.into()),
            (SettingId::LowBatteryPrompt, false.into()),
            (SettingId::AutoPowerOffPrompt, false.into()),
            (SettingId::NoiseCancelingPrompt, true.into()),
            (SettingId::ListeningModePrompt, true.into()),
            (SettingId::FirmwareVersionLeft, "81.00".into()),
            (SettingId::FirmwareVersionRight, "81.00".into()),
            (SettingId::SerialNumber, "1301000000000000".into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn delete_alarm_shifts_indices() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD1301,
            HashMap::from([
                (
                    packet::Command([1, 1]),
                    packet::Inbound::new(
                        packet::Command([1, 1]),
                        vec![
                            1, 1, 9, 9, 56, 49, 46, 48, 48, 56, 49, 46, 48, 48, 49, 51, 48, 49, 48,
                            48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 223, 186, 112, 217, 191,
                            144, 48, 49, 46, 48, 48, 0, 0, 0, 0, 6, 0xdd, 0x66, 0x00, 0x00, 255,
                            255, 255, 255, 255, 255, 255, 255, 0, 30, 0, 1, 255, 255, 255, 0, 0, 0,
                            0, 0, 50, 17, 1, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 203, 49, 61, 0,
                            0, 0, 0, 0, 2, 2, 1, 128, 0, 0, 0, 0, 0, 0, 4, 3, 2, 128, 0, 0, 0, 0,
                            0, 0, 4, 4, 3, 128, 0, 0, 0, 0, 0, 0, 4, 5, 4, 128, 0, 0, 0, 0, 0, 0,
                            4, 1, 0, 0, 1, 8, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0,
                        ],
                    ),
                ),
                (
                    packets::inbound::AutoStopTimerPacket::COMMAND,
                    packet::Inbound::new(
                        packets::inbound::AutoStopTimerPacket::COMMAND,
                        vec![0, 30, 0, 1, 60, 0, 0, 0],
                    ),
                ),
                (
                    packets::inbound::AlarmsPacket::COMMAND,
                    packet::Inbound::new(packets::inbound::AlarmsPacket::COMMAND, vec![]),
                ),
            ]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        // create the alarms before deleting one, since if we do it in one go,
        // it may optimize and skip creating one rather than deleting
        device
            .assert_set_settings_response(
                vec![
                    (SettingId::CreateAlarm, true.into()),
                    (SettingId::CreateAlarm, true.into()),
                    // volume is different for alarm 2 to distinguish the two
                    (SettingId::Alarm2Volume, 51.into()),
                ],
                vec![
                    packet::Outbound::new(
                        packet::Command([20, 129]), // set alarm
                        vec![0, 0, 0, 0, 0, 0, 50, 10],
                    ),
                    packet::Outbound::new(
                        packet::Command([20, 129]), // set alarm
                        vec![1, 0, 0, 0, 0, 0, 51, 10],
                    ),
                    packet::Outbound::new(
                        packet::Command([20, 1]), // get alarms
                        Vec::new(),
                    ),
                ],
            )
            .await;
        device
            .assert_set_settings_response(
                vec![(SettingId::DeleteAlarm1, true.into())],
                vec![
                    packet::Outbound::new(
                        packet::Command([20, 130]), // delete alarm
                        vec![0],
                    ),
                    packet::Outbound::new(
                        packet::Command([20, 1]), // get alarms
                        Vec::new(),
                    ),
                ],
            )
            .await;

        device.assert_setting_values(vec![(SettingId::Alarm1Volume, 51.into())]);
    }

    mod dual_connections {
        use std::{sync::Arc, time::Duration};

        use macaddr::MacAddr6;

        use super::*;
        use crate::{
            devices::soundcore::common::structures::DualConnectionsDevice,
            storage::OpenSCQ30Database,
        };

        fn packet_responses(
            dual_connections_devices: Vec<DualConnectionsDevice>,
        ) -> HashMap<packet::Command, packet::Inbound> {
            HashMap::from([
                (
                    RequestState::COMMAND,
                    D1301StateUpdatePacket::default().to_packet(),
                ),
                (
                    packets::inbound::AutoStopTimerPacket::COMMAND,
                    packets::inbound::AutoStopTimerPacket(structures::AutoStopTimer::default())
                        .to_packet(),
                ),
                (
                    packets::inbound::AlarmsPacket::COMMAND,
                    packets::inbound::AlarmsPacket(Vec::new()).to_packet(),
                ),
                (
                    DualConnectionsDevicePacket::COMMAND,
                    DualConnectionsDevicePacket {
                        total_packets: 1,
                        current_packet_index: 1,
                        devices: dual_connections_devices,
                    }
                    .to_packet(),
                ),
                // captured from a real device: echoes the value followed by a 0 status byte
                (
                    packet::Command([0x0b, 0x84]),
                    packet::Inbound::new(packet::Command([0x0b, 0x84]), vec![1, 0]),
                ),
            ])
        }

        fn host(last_octet: u8, is_connected: bool) -> DualConnectionsDevice {
            DualConnectionsDevice {
                is_connected,
                mac_address: MacAddr6::new(0, 0, 0, 0, 0, last_octet),
                name: format!("Device {last_octet}"),
            }
        }

        async fn connect(
            database: Arc<OpenSCQ30Database>,
            dual_connections_devices: Vec<DualConnectionsDevice>,
        ) -> TestSoundcoreDevice {
            TestSoundcoreDevice::new_with_database(
                super::super::device_registry,
                DeviceModel::SoundcoreD1301,
                packet_responses(dual_connections_devices),
                SoundcoreDeviceConfig::default(),
                HashMap::<packet::Command, Duration>::new(),
                database,
            )
            .await
        }

        async fn database() -> Arc<OpenSCQ30Database> {
            Arc::new(OpenSCQ30Database::new_in_memory().await.unwrap())
        }

        #[tokio::test(start_paused = true)]
        async fn is_off_when_unknown() {
            let device = connect(database().await, vec![host(1, true), host(2, false)]).await;
            device.assert_setting_values([(SettingId::DualConnections, false.into())]);
        }

        #[tokio::test(start_paused = true)]
        async fn is_on_when_two_devices_are_connected() {
            let device = connect(database().await, vec![host(1, true), host(2, true)]).await;
            device.assert_setting_values([(SettingId::DualConnections, true.into())]);
        }

        #[tokio::test(start_paused = true)]
        async fn enabling_sends_packet() {
            let mut device = connect(database().await, vec![host(1, true)]).await;
            device
                .assert_set_settings_response(
                    vec![(SettingId::DualConnections, true.into())],
                    vec![packet::outbound::set_dual_connections_enabled(true)],
                )
                .await;
            device.assert_setting_values([(SettingId::DualConnections, true.into())]);
        }

        #[tokio::test(start_paused = true)]
        async fn enabled_is_reapplied_on_reconnect() {
            let database = database().await;
            let mut device = connect(database.clone(), vec![host(1, true)]).await;
            device
                .set_settings(vec![(SettingId::DualConnections, true.into())])
                .await;
            drop(device);

            // simulate the earbuds having been powered off, which disables dual connections
            let device = connect(database, vec![host(1, true)]).await;
            device.assert_setting_values([(SettingId::DualConnections, true.into())]);
        }

        #[tokio::test(start_paused = true)]
        async fn enabling_while_already_on_is_reapplied_on_reconnect() {
            let database = database().await;
            // two connected devices: already shown as enabled, but nothing has been saved yet
            let mut device = connect(database.clone(), vec![host(1, true), host(2, true)]).await;
            device
                .set_settings(vec![(SettingId::DualConnections, true.into())])
                .await;
            drop(device);

            let device = connect(database, vec![host(1, true)]).await;
            device.assert_setting_values([(SettingId::DualConnections, true.into())]);
        }

        #[tokio::test(start_paused = true)]
        async fn disabled_is_not_reapplied_on_reconnect() {
            let database = database().await;
            let mut device = connect(database.clone(), vec![host(1, true)]).await;
            device
                .set_settings(vec![(SettingId::DualConnections, true.into())])
                .await;
            device
                .set_settings(vec![(SettingId::DualConnections, false.into())])
                .await;
            drop(device);

            let device = connect(database, vec![host(1, true)]).await;
            device.assert_setting_values([(SettingId::DualConnections, false.into())]);
        }
    }
}

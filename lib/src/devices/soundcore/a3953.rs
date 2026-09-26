use std::collections::HashMap;

use crate::devices::soundcore::{
    a3953::{packets::A3953StateUpdatePacket, state::A3953State},
    common::{
        self,
        macros::soundcore_device,
        modules::{
            auto_power_off::AutoPowerOffDuration,
            button_configuration::{
                ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings, COMMON_ACTIONS,
            },
            equalizer::common_settings_type_2,
        },
        packet::{
            inbound::TryToPacket,
            outbound::{RequestState, ToPacket},
        },
        structures::button_configuration::{
            ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
        },
    },
};

mod modules;
mod packets;
mod state;
mod structures;

// button_id per press kind (2/0/1/5 for single/double/long/triple) matches this project's
// existing convention for every other device on this shared command, and was cross-checked byte
// for byte against CmmBtCmdService.t2's ViewModel caller (BaseControllerSelectVM.sendClickTypeCmd).
// Single-press was confirmed end to end on real hardware (2026-09-14): the write packet decoded
// exactly as predicted (command [4,129], side=0 for left, button_id=2, TwsLowBits-packed action
// byte), the read-back matched, and the physical button's behavior changed on the correct earbud
// only. Double/long/triple use the identical mechanism and the same button_id convention already
// verified on 4+ other devices, but weren't independently exercised on this device.
pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<8, 4> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        ignore_enabled_flag: true,
        set_button_action_command_override: None,
        setting_id_override: None,
        order: [
            Button::LeftSinglePress,
            Button::RightSinglePress,
            Button::LeftDoublePress,
            Button::RightDoublePress,
            Button::LeftLongPress,
            Button::RightLongPress,
            Button::LeftTriplePress,
            Button::RightTriplePress,
        ],
        settings: [
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 5,
                press_kind: ButtonPressKind::Triple,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::IndividualDisable,
            },
        ],
    };

soundcore_device!(
    A3953State,
    async |packet_io| {
        let state_update_packet: A3953StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(A3953State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.serial_number_and_dual_firmware_version();
        builder.tws_status();
        // Only ever observed at 5/5 (fully charged); assumed max_level 5 to match the structurally
        // similar A3947 (Liberty 4 NC) until a partial-charge capture confirms or corrects this.
        builder.dual_battery(5);
        builder.a3953_sound_modes();
        builder.ambient_sound_mode_cycle();
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.reset_button_configuration::<A3953StateUpdatePacket>(RequestState.to_packet());
        builder.wearing_detection();
        // Same caveat as dual_battery above: the device's own S() clamp permits 0-9, but no
        // partial-charge capture exists to confirm whether 5 is really this device's max.
        builder.case_battery_level(5);
        builder.ldac();
        builder.auto_power_off(AutoPowerOffDuration::ten_twenty_thirty_sixty());
        builder.wearing_tone();
        builder.low_battery_prompt();
        builder.a3953_misc_toggles();
        builder.dual_connections();
        builder.a3953_spatial_audio();
        builder.a3953_press_sensitivity();
        builder.a3953_equalizer(common_settings_type_2()).await;
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3953StateUpdatePacket::default().to_packet(),
        )])
    },
);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
            packet,
            structures::CommonVolumeAdjustments,
        },
        settings::{SettingId, Value},
    };

    #[tokio::test(start_paused = true)]
    async fn parses_known_packet() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3953,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 5, 5, 0, 0, 48, 51, 46, 50, 51, 48, 51, 46, 50, 51, 51, 57, 53, 51,
                        53, 52, 67, 54, 51, 51, 67, 67, 69, 69, 69, 56, 0, 0, 120, 120, 120, 120,
                        120, 120, 120, 120, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 0, 60, 60, 60, 60, 60, 60, 60, 60, 0, 0, 60, 60, 60, 60, 60, 60, 60,
                        60, 0, 0, 0, 0, 0, 0, 0, 60, 60, 60, 60, 60, 60, 60, 60, 0, 0, 60, 60, 60,
                        60, 60, 60, 60, 60, 0, 0, 0, 0, 18, 17, 102, 17, 102, 17, 52, 17, 52, 17,
                        36, 17, 36, 17, 82, 17, 83, 3, 2, 48, 0, 0, 1, 1, 0, 0, 0, 0, 255, 255, 1,
                        1, 0, 5, 0, 0, 0, 1, 2, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 120, 255, 50, 0, 255,
                        255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([
            (SettingId::TwsStatus, "Connected".into()),
            (SettingId::HostDevice, "Right".into()),
            (SettingId::BatteryLevelLeft, "5/5".into()),
            (SettingId::BatteryLevelRight, "5/5".into()),
            (SettingId::IsChargingLeft, "No".into()),
            (SettingId::IsChargingRight, "No".into()),
            (SettingId::FirmwareVersionLeft, "03.23".into()),
            (SettingId::FirmwareVersionRight, "03.23".into()),
            (SettingId::SerialNumber, "395354C633CCEEE8".into()),
            (SettingId::AmbientSoundMode, "Normal".into()),
            (SettingId::WindNoiseSuppression, true.into()),
            (SettingId::NormalModeInCycle, false.into()),
            (SettingId::TransparencyModeInCycle, true.into()),
            (SettingId::NoiseCancelingModeInCycle, true.into()),
            (SettingId::WearingDetection, true.into()),
            (SettingId::CaseBatteryLevel, "5/5".into()),
            (SettingId::Ldac, false.into()),
            (SettingId::AutoPowerOff, "30m".into()),
            (SettingId::WearingTone, true.into()),
            (SettingId::PressSensitivity, 0.into()),
            (SettingId::LowBatteryPrompt, true.into()),
            (SettingId::AmbientSoundPrompt, true.into()),
            (SettingId::DualConnections, false.into()),
            (SettingId::SpatialAudio, false.into()),
            (SettingId::SpatialAudioMode, "Music".into()),
            (SettingId::SpatialAudioMusicMode, "Fixed".into()),
            (SettingId::VolumeAdjustments, Value::I16Vec(vec![0; 8])),
        ]);
    }

    // Same real capture as `parses_known_packet`. Verifies the outbound custom EQ write packet
    // byte for byte: command `[0x03, 0x87]` (decompiled `Cmm2CmdData.p1`, reached via
    // `A3953CmdService.c5()` -> `CmmBtCmdService.z5`/`v5`), the plain custom EQ values, the
    // Hear-ID-uninitialized `255, 255` sentinel (this capture's own Hear ID data, round-tripped
    // unchanged from what `parses_known_packet` decodes: not enabled, all-`-60` stored values,
    // `time=0`, `hear_id_type=Initial`, `favorite_music_genre=0`), and the DRC-transformed copy of
    // the new custom EQ values as the trailing "real eq" block (see
    // `common::structures::VolumeAdjustments::apply_drc`, itself a verbatim transcription of the
    // decompiled app's `HearId2Utils.b`).
    #[tokio::test(start_paused = true)]
    async fn sets_custom_equalizer() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3953,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 5, 5, 0, 0, 48, 51, 46, 50, 51, 48, 51, 46, 50, 51, 51, 57, 53, 51,
                        53, 52, 67, 54, 51, 51, 67, 67, 69, 69, 69, 56, 0, 0, 120, 120, 120, 120,
                        120, 120, 120, 120, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 0, 60, 60, 60, 60, 60, 60, 60, 60, 0, 0, 60, 60, 60, 60, 60, 60, 60,
                        60, 0, 0, 0, 0, 0, 0, 0, 60, 60, 60, 60, 60, 60, 60, 60, 0, 0, 60, 60, 60,
                        60, 60, 60, 60, 60, 0, 0, 0, 0, 18, 17, 102, 17, 102, 17, 52, 17, 52, 17,
                        36, 17, 36, 17, 82, 17, 83, 3, 2, 48, 0, 0, 1, 1, 0, 0, 0, 0, 255, 255, 1,
                        1, 0, 5, 0, 0, 0, 1, 2, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 120, 255, 50, 0, 255,
                        255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        // Bands 8 and 9 aren't settable through `SettingId::VolumeAdjustments` (only the 8 visible
        // bands are), so the equalizer module leaves them at 0 rather than the DRC transform's own
        // internal defaults for those two positions.
        let new_adjustments =
            CommonVolumeAdjustments::<10>::new([60, -60, 60, -60, 60, -60, 60, -60, 0, 0]);
        // Decoded from this capture's own Hear ID block by `parses_known_packet`'s sibling
        // assertions; round-tripped unchanged since this project doesn't expose Hear ID for
        // editing.
        let stored_hear_id_adjustments = CommonVolumeAdjustments::<10>::new([
            -60, -60, -60, -60, -60, -60, -60, -60, -120, -120,
        ]);

        let mut expected_body = Vec::new();
        expected_body.extend([254u8, 254u8]); // preset id: custom (0xFEFE)
        expected_body.extend([0u8, 0u8]); // favorite_music_genre (unchanged from the capture)
        expected_body.extend(new_adjustments.bytes());
        expected_body.extend(new_adjustments.bytes());
        expected_body.extend([255u8, 255u8]); // Hear ID not initialized in this capture
        expected_body.push(0); // hear_id.is_enabled: always disabled on write
        expected_body.extend(stored_hear_id_adjustments.bytes());
        expected_body.extend(stored_hear_id_adjustments.bytes());
        expected_body.extend(0u32.to_be_bytes()); // hear_id.time
        expected_body.push(0); // hear_id_type: Initial
        expected_body.extend(stored_hear_id_adjustments.bytes());
        expected_body.extend(stored_hear_id_adjustments.bytes());
        expected_body.extend(new_adjustments.apply_drc().bytes());
        expected_body.extend(new_adjustments.apply_drc().bytes());
        expected_body.push(0); // trailing byte

        device
            .assert_set_settings_response(
                vec![(
                    SettingId::VolumeAdjustments,
                    Value::I16Vec(vec![60, -60, 60, -60, 60, -60, 60, -60]),
                )],
                vec![packet::Outbound::new(
                    packet::Command([3, 135]),
                    expected_body,
                )],
            )
            .await;
    }

    // The three tests below are real captures of the same device, differing only in byte 121 (the
    // ambient sound mode toggled in the official app between captures): 2 (Normal), 0 (Noise
    // Canceling), and 1 (Transparency). See a3953/structures.rs for the byte citation.
    #[tokio::test(start_paused = true)]
    async fn parses_noise_canceling_mode() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3953,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 5, 5, 0, 0, 48, 51, 46, 50, 51, 48, 51, 46, 50, 51, 51, 57, 53, 51,
                        53, 52, 67, 54, 51, 51, 67, 67, 69, 69, 69, 56, 0, 0, 120, 120, 120, 120,
                        120, 120, 120, 120, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 0, 60, 60, 60, 60, 60, 60, 60, 60, 0, 0, 60, 60, 60, 60, 60, 60, 60,
                        60, 0, 0, 0, 0, 0, 0, 0, 60, 60, 60, 60, 60, 60, 60, 60, 0, 0, 60, 60, 60,
                        60, 60, 60, 60, 60, 0, 0, 0, 0, 18, 17, 102, 17, 102, 17, 52, 17, 52, 17,
                        36, 17, 36, 17, 82, 17, 83, 3, 0, 48, 0, 0, 1, 1, 0, 0, 0, 0, 255, 255, 1,
                        1, 0, 5, 0, 0, 0, 1, 2, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 120, 255, 50, 0, 255,
                        255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([(SettingId::AmbientSoundMode, "NoiseCanceling".into())]);
    }

    #[tokio::test(start_paused = true)]
    async fn parses_transparency_mode() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3953,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        1, 1, 5, 5, 0, 0, 48, 51, 46, 50, 51, 48, 51, 46, 50, 51, 51, 57, 53, 51,
                        53, 52, 67, 54, 51, 51, 67, 67, 69, 69, 69, 56, 0, 0, 120, 120, 120, 120,
                        120, 120, 120, 120, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                        255, 0, 60, 60, 60, 60, 60, 60, 60, 60, 0, 0, 60, 60, 60, 60, 60, 60, 60,
                        60, 0, 0, 0, 0, 0, 0, 0, 60, 60, 60, 60, 60, 60, 60, 60, 0, 0, 60, 60, 60,
                        60, 60, 60, 60, 60, 0, 0, 0, 0, 18, 17, 102, 17, 102, 17, 52, 17, 52, 17,
                        36, 17, 36, 17, 82, 17, 83, 3, 1, 48, 0, 0, 1, 1, 0, 0, 0, 0, 255, 255, 1,
                        1, 0, 5, 0, 0, 0, 1, 2, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 120, 255, 50, 0, 255,
                        255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values([(SettingId::AmbientSoundMode, "Transparency".into())]);
    }

    // Real capture: two unsolicited [0x0B, 0x01] packets pushed by the device after connecting,
    // confirming A3953 uses the same wire format as common::packet::inbound::DualConnectionsDevicePacket.
    // The second packet's lone entry is padded 2 bytes short of what its own length byte implies,
    // which is what motivated DualConnectionsDevice::take's clamp to available input.
    #[test]
    fn parses_real_dual_connections_device_list_capture() {
        use nom_language::error::VerboseError;

        use crate::devices::soundcore::common::packet::inbound::{
            DualConnectionsDevicePacket, FromPacketBody,
        };

        let packet_1 = [
            2, 1, 40, 1, 175, 149, 25, 87, 47, 132, 83, 101, 97, 110, 226, 128, 153, 115, 32, 77,
            97, 99, 66, 111, 111, 107, 32, 80, 114, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0,
            117, 224, 41, 200, 87, 96, 83, 101, 97, 110, 226, 128, 153, 115, 32, 105, 80, 104, 111,
            110, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 113, 159, 29, 98,
            59, 28, 78, 111, 107, 105, 97, 32, 50, 55, 56, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 198, 14, 117, 171, 169, 60, 78, 105, 110, 116,
            101, 110, 100, 111, 32, 83, 119, 105, 116, 99, 104, 32, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        let (remaining_1, parsed_1) =
            DualConnectionsDevicePacket::take::<VerboseError<_>>(&packet_1).unwrap();
        assert_eq!(remaining_1.len(), 0);
        assert_eq!(parsed_1.total_packets, 2);
        assert_eq!(parsed_1.current_packet_index, 1);
        let names: Vec<_> = parsed_1.devices.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(
            names,
            [
                "Sean\u{2019}s MacBook Pro",
                "Sean\u{2019}s iPhone",
                "Nokia 2780",
                "Nintendo Switch 2",
            ]
        );
        assert!(parsed_1.devices[0].is_connected);
        assert!(!parsed_1.devices[1].is_connected);
        assert!(!parsed_1.devices[2].is_connected);
        assert!(!parsed_1.devices[3].is_connected);

        let packet_2 = [
            2, 2, 40, 0, 96, 137, 137, 109, 73, 184, 83, 101, 97, 110, 226, 128, 153, 115, 32, 105,
            80, 97, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let (remaining_2, parsed_2) =
            DualConnectionsDevicePacket::take::<VerboseError<_>>(&packet_2).unwrap();
        assert_eq!(remaining_2.len(), 0);
        assert_eq!(parsed_2.total_packets, 2);
        assert_eq!(parsed_2.current_packet_index, 2);
        assert_eq!(parsed_2.devices.len(), 1);
        assert_eq!(parsed_2.devices[0].name, "Sean\u{2019}s iPad");
        assert!(!parsed_2.devices[0].is_connected);
    }
}

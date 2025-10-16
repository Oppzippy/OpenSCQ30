use std::collections::HashMap;

use crate::devices::soundcore::{
    a3948::{packets::inbound::A3948StateUpdatePacket, state::A3948State},
    common::{
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
};

mod packets;
mod state;

soundcore_device!(
    A3948State,
    A3948StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3948State, A3948StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.equalizer_with_drc().await;

        builder.button_configuration_v2(&BUTTON_CONFIGURATION_SETTINGS);

        builder.touch_tone();

        builder.serial_number_and_dual_firmware_version();
        builder.tws_status();
        builder.dual_battery(5);
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3948StateUpdatePacket::default().bytes(),
        )])
    },
);

pub const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<6, 3> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false,
        order: [
            Button::LeftSinglePress,
            Button::RightSinglePress,
            Button::LeftDoublePress,
            Button::RightDoublePress,
            Button::LeftLongPress,
            Button::RightLongPress,
        ],
        settings: [
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_TWS_ACTIONS,
                disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_TWS_ACTIONS,
                disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::TwsLowBits,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_TWS_ACTIONS,
                disable_mode: ButtonDisableMode::DisablingOneSideDisablesOther,
            },
        ],
    };

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            device::test_utils::TestSoundcoreDevice,
            packet::{Command, Direction, Packet},
        },
        settings::SettingId,
    };

    #[tokio::test(start_paused = true)]
    async fn test_new_with_example_state_update_packet() {
        let device = TestSoundcoreDevice::new_with_packet_responses(
            super::device_registry,
            DeviceModel::SoundcoreA3948,
            HashMap::from([(
                Command([1, 1]),
                Packet {
                    direction: Direction::Inbound,
                    command: Command([1, 1]),
                    body: vec![
                        0, 0, 5, 255, 0, 0, 50, 49, 46, 53, 54, 0, 0, 0, 0, 0, 51, 57, 52, 56, 55,
                        49, 48, 54, 56, 54, 54, 54, 65, 69, 70, 48, 19, 0, 90, 100, 130, 140, 140,
                        130, 120, 90, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 1, 241, 1, 255, 1,
                        98, 1, 246, 1, 54, 1, 243, 255, 255, 255, 49, 0, 1, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    ],
                },
            )]),
        )
        .await;
        device.assert_setting_values([
            (SettingId::FirmwareVersionLeft, "21.56".into()),
            (SettingId::FirmwareVersionRight, "".into()),
        ]);
    }
}

use std::collections::HashMap;

use crate::devices::soundcore::{
    a3930::{packets::A3930StateUpdatePacket, state::A3930State},
    common::{
        macros::soundcore_device,
        modules::{
            button_configuration_v2::{
                ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings,
                COMMON_ACTIONS_WITHOUT_SOUND_MODES,
            },
            sound_modes::AvailableSoundModes,
        },
        packet::{
            inbound::{SerialNumberAndFirmwareVersion, TryIntoInboundPacket},
            outbound::{
                OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersion, RequestState,
            },
        },
        structures::{
            AmbientSoundMode,
            button_configuration_v2::{
                ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
            },
        },
    },
};

mod packets;
mod state;

soundcore_device!(
    A3930State,
    A3930StateUpdatePacket,
    async |packet_io| {
        let state_update_packet: A3930StateUpdatePacket = packet_io
            .send_with_response(&RequestState::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersion = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersion::new().into())
            .await?
            .try_into_inbound_packet()?;
        Ok(A3930State::new(state_update_packet, sn_and_firmware))
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.sound_modes(AvailableSoundModes {
            ambient_sound_modes: vec![AmbientSoundMode::Normal, AmbientSoundMode::Transparency],
            transparency_modes: Vec::new(),
            noise_canceling_modes: Vec::new(),
        });
        builder.equalizer_with_custom_hear_id().await;
        builder.button_configuration_v2(&BUTTON_CONFIGURATION_SETTINGS);
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestState::COMMAND,
                A3930StateUpdatePacket::default().bytes(),
            ),
            (
                RequestSerialNumberAndFirmwareVersion::COMMAND,
                SerialNumberAndFirmwareVersion::default().bytes(),
            ),
        ])
    },
);

const BUTTON_CONFIGURATION_SETTINGS: ButtonConfigurationSettings<6, 3> =
    ButtonConfigurationSettings {
        supports_set_all_packet: false, // unknown so false to be safe
        use_enabled_flag_to_disable: true,
        order: [
            Button::LeftDoublePress,
            Button::LeftLongPress,
            Button::RightDoublePress,
            Button::RightLongPress,
            Button::LeftSinglePress,
            Button::RightSinglePress,
        ],
        settings: [
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::Single,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 0,
                press_kind: ButtonPressKind::Double,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::NotDisablable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::Single,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::NotDisablable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::Single,
                    action_kind: ActionKind::Single,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS_WITHOUT_SOUND_MODES,
                disable_mode: ButtonDisableMode::NotDisablable,
            },
        ],
    };

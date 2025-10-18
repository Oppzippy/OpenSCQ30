use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3931::{packets::A3931StateUpdatePacket, state::A3931State},
        common::{
            macros::soundcore_device,
            modules::{
                button_configuration::{
                    ButtonConfigurationSettings, ButtonDisableMode, ButtonSettings, COMMON_ACTIONS,
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
                AmbientSoundMode, NoiseCancelingMode, TransparencyMode,
                button_configuration::{
                    ActionKind, Button, ButtonParseSettings, ButtonPressKind, EnabledFlagKind,
                },
            },
        },
    },
    i18n::fl,
};

mod packets;
mod state;

soundcore_device!(
    A3931State,
    A3931StateUpdatePacket,
    async |packet_io| {
        let state_update_packet: A3931StateUpdatePacket = packet_io
            .send_with_response(&RequestState::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersion = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersion::new().into())
            .await?
            .try_into_inbound_packet()?;
        Ok(A3931State::new(state_update_packet, sn_and_firmware))
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
                NoiseCancelingMode::Outdoor,
                NoiseCancelingMode::Indoor,
            ],
        });
        builder.equalizer_with_drc().await;
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.auto_power_off(AutoPowerOffDuration::VARIANTS);
        builder.touch_tone();
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestState::COMMAND,
                A3931StateUpdatePacket::default().bytes(),
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
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::NotDisablable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::Single,
                    action_kind: ActionKind::TwsLowBits,
                },
                button_id: 1,
                press_kind: ButtonPressKind::Long,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::NotDisablable,
            },
            ButtonSettings {
                parse_settings: ButtonParseSettings {
                    enabled_flag_kind: EnabledFlagKind::Single,
                    action_kind: ActionKind::Single,
                },
                button_id: 2,
                press_kind: ButtonPressKind::Single,
                available_actions: COMMON_ACTIONS,
                disable_mode: ButtonDisableMode::NotDisablable,
            },
        ],
    };

#[derive(IntoStaticStr, VariantArray)]
#[allow(clippy::enum_variant_names)]
enum AutoPowerOffDuration {
    #[strum(serialize = "5m")]
    FiveMinutes,
    #[strum(serialize = "10m")]
    TenMinutes,
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[strum(serialize = "60m")]
    SixtyMinutes,
}

impl Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            Self::FiveMinutes => fl!("x-minutes", minutes = 5),
            Self::TenMinutes => fl!("x-minutes", minutes = 10),
            Self::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            Self::SixtyMinutes => fl!("x-minutes", minutes = 60),
        }
    }
}

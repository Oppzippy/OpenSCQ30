use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3031::{packets::A3031StateUpdatePacket, state::A3031State},
        common::{
            macros::soundcore_device,
            modules::{
                button_configuration_v2::COMMON_SETTINGS as BUTTON_CONFIGURATION_SETTINGS,
                sound_modes::AvailableSoundModes,
            },
            packet::{
                inbound::{SerialNumberAndFirmwareVersion, TryIntoInboundPacket},
                outbound::{
                    OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersion, RequestState,
                },
            },
            structures::{AmbientSoundMode, NoiseCancelingMode},
        },
    },
    i18n::fl,
};

mod packets;
mod state;

soundcore_device!(
    A3031State,
    A3031StateUpdatePacket,
    async |packet_io| {
        let state_update_packet: A3031StateUpdatePacket = packet_io
            .send_with_response(&RequestState::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersion = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersion::new().into())
            .await?
            .try_into_inbound_packet()?;
        Ok(A3031State::new(state_update_packet, sn_and_firmware))
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
        builder.touch_tone();
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestState::COMMAND,
                A3031StateUpdatePacket::default().bytes(),
            ),
            (
                RequestSerialNumberAndFirmwareVersion::COMMAND,
                SerialNumberAndFirmwareVersion::default().bytes(),
            ),
        ])
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

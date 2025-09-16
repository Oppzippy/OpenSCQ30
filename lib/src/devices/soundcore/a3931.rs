use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3931::{packets::A3931StateUpdatePacket, state::A3931State},
        common::{
            macros::soundcore_device,
            modules::sound_modes::AvailableSoundModes,
            packet::{
                inbound::{SerialNumberAndFirmwareVersion, TryIntoInboundPacket},
                outbound::{
                    OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersion, RequestState,
                },
            },
            structures::{AmbientSoundMode, TransparencyMode},
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
            ambient_sound_modes: vec![AmbientSoundMode::Normal, AmbientSoundMode::Transparency],
            transparency_modes: vec![
                TransparencyMode::FullyTransparent,
                TransparencyMode::VocalMode,
            ],
            noise_canceling_modes: Vec::new(),
        });
        builder.equalizer_with_drc().await;
        builder.button_configuration();
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

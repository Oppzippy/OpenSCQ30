use std::collections::HashMap;

use openscq30_i18n::Translate;
use strum::{IntoStaticStr, VariantArray};

use crate::{
    devices::soundcore::{
        a3931::{packets::A3931StateUpdatePacket, state::A3931State},
        standard::{
            macros::soundcore_device,
            modules::sound_modes::AvailableSoundModes,
            packet::{
                inbound::{SerialNumberAndFirmwareVersionUpdatePacket, TryIntoInboundPacket},
                outbound::{
                    OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersionPacket,
                    RequestStatePacket,
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
            .send_with_response(&RequestStatePacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersionUpdatePacket = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersionPacket::new().into())
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
        builder.tws_status();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestStatePacket::COMMAND,
                A3931StateUpdatePacket::default().bytes(),
            ),
            (
                RequestSerialNumberAndFirmwareVersionPacket::COMMAND,
                SerialNumberAndFirmwareVersionUpdatePacket::default().bytes(),
            ),
        ])
    },
);

#[repr(u8)]
#[derive(IntoStaticStr, VariantArray)]
pub enum AutoPowerOffDuration {
    #[strum(serialize = "5m")]
    FiveMinutes = 0,
    #[strum(serialize = "10m")]
    TenMinutes = 1,
    #[strum(serialize = "30m")]
    ThirtyMinutes = 2,
    #[strum(serialize = "1h")]
    OneHour = 3,
}

impl Translate for AutoPowerOffDuration {
    fn translate(&self) -> String {
        match self {
            AutoPowerOffDuration::FiveMinutes => fl!("x-minutes", minutes = 5),
            AutoPowerOffDuration::TenMinutes => fl!("x-minutes", minutes = 10),
            AutoPowerOffDuration::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            AutoPowerOffDuration::OneHour => fl!("x-minutes", minutes = 60),
        }
    }
}

use std::collections::HashMap;

use crate::devices::soundcore::{
    a3951::{packets::A3951StateUpdatePacket, state::A3951State},
    common::{
        macros::soundcore_device,
        modules::sound_modes::AvailableSoundModes,
        packet::{
            inbound::{SerialNumberAndFirmwareVersionUpdatePacket, TryIntoInboundPacket},
            outbound::{
                OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersionPacket,
                RequestStatePacket,
            },
        },
        structures::{AmbientSoundMode, NoiseCancelingMode, TransparencyMode},
    },
};

mod packets;
mod state;

soundcore_device!(
    A3951State,
    A3951StateUpdatePacket,
    async |packet_io| {
        let state_update_packet: A3951StateUpdatePacket = packet_io
            .send_with_response(&RequestStatePacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersionUpdatePacket = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersionPacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        Ok(A3951State::new(state_update_packet, sn_and_firmware))
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
                NoiseCancelingMode::Indoor,
                NoiseCancelingMode::Outdoor,
                NoiseCancelingMode::Custom,
            ],
        });
        builder.equalizer_with_custom_hear_id().await;
        builder.button_configuration();
        builder.touch_tone();
        builder.tws_status();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestStatePacket::COMMAND,
                A3951StateUpdatePacket::default().bytes(),
            ),
            (
                RequestSerialNumberAndFirmwareVersionPacket::COMMAND,
                SerialNumberAndFirmwareVersionUpdatePacket::default().bytes(),
            ),
        ])
    },
);

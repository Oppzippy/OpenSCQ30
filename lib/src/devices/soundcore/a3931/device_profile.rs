use std::collections::HashMap;

use crate::devices::soundcore::standard::{
    macros::soundcore_device,
    modules::sound_modes::AvailableSoundModes,
    packets::{
        inbound::{SerialNumberAndFirmwareVersionUpdatePacket, TryIntoInboundPacket},
        outbound::{OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersionPacket, RequestStatePacket},
    },
    structures::{AmbientSoundMode, TransparencyMode},
};

use super::{packets::A3931StateUpdatePacket, state::A3931State};

soundcore_device!(
    A3931State,
    A3931StateUpdatePacket,
    async |packet_io| {
        let state_update_packet: A3931StateUpdatePacket = packet_io
            .send(&RequestStatePacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersionUpdatePacket = packet_io
            .send(&RequestSerialNumberAndFirmwareVersionPacket::new().into())
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

use std::collections::HashMap;

use crate::devices::soundcore::standard::{
    macros::soundcore_device,
    modules::sound_modes::AvailableSoundModes,
    packets::{
        inbound::{SerialNumberAndFirmwareVersionUpdatePacket, TryIntoInboundPacket},
        outbound::{OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersionPacket, RequestStatePacket},
    },
    structures::{AmbientSoundMode, NoiseCancelingMode},
};

use super::{packets::A3031StateUpdatePacket, state::A3031State};

soundcore_device!(
    A3031State,
    A3031StateUpdatePacket,
    async |packet_io| {
        let state_update_packet: A3031StateUpdatePacket = packet_io
            .send(&RequestStatePacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersionUpdatePacket = packet_io
            .send(&RequestSerialNumberAndFirmwareVersionPacket::new().into())
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
        builder.tws_status();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestStatePacket::COMMAND,
                A3031StateUpdatePacket::default().bytes(),
            ),
            (
                RequestSerialNumberAndFirmwareVersionPacket::COMMAND,
                SerialNumberAndFirmwareVersionUpdatePacket::default().bytes(),
            ),
        ])
    },
);

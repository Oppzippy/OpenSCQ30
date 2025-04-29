use std::collections::HashMap;

use crate::devices::soundcore::standard::{
    macros::soundcore_device,
    packets::{
        inbound::{SerialNumberAndFirmwareVersionUpdatePacket, TryIntoInboundPacket},
        outbound::{
            OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersionPacket, RequestStatePacket,
        },
    },
};

use super::{packets::A3926StateUpdatePacket, state::A3926State};

soundcore_device!(
    A3926State,
    A3926StateUpdatePacket,
    async |packet_io| {
        let state_update_packet: A3926StateUpdatePacket = packet_io
            .send(&RequestStatePacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersionUpdatePacket = packet_io
            .send(&RequestSerialNumberAndFirmwareVersionPacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        Ok(A3926State::new(state_update_packet, sn_and_firmware))
    },
    async |builder| {
        builder.module_collection().add_state_update();
        // TODO confirm that this doesn't actually have sound modes and the below code is wrong
        // builder.sound_modes(AvailableSoundModes {
        //     ambient_sound_modes: vec![
        //         AmbientSoundMode::Normal,
        //         AmbientSoundMode::Transparency,
        //         AmbientSoundMode::NoiseCanceling,
        //     ],
        //     transparency_modes: Vec::new(),
        //     noise_canceling_modes: vec![
        //         NoiseCancelingMode::Transport,
        //         NoiseCancelingMode::Indoor,
        //         NoiseCancelingMode::Outdoor,
        //     ],
        // });
        builder.equalizer_with_basic_hear_id().await;
        builder.button_configuration();
        builder.tws_status();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestStatePacket::COMMAND,
                A3926StateUpdatePacket::default().bytes(),
            ),
            (
                RequestSerialNumberAndFirmwareVersionPacket::COMMAND,
                SerialNumberAndFirmwareVersionUpdatePacket::default().bytes(),
            ),
        ])
    },
);

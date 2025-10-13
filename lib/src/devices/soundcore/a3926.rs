use std::collections::HashMap;

use crate::devices::soundcore::{
    a3926::{packets::A3926StateUpdatePacket, state::A3926State},
    common::{
        macros::soundcore_device,
        packet::{
            inbound::{SerialNumberAndFirmwareVersion, TryIntoInboundPacket},
            outbound::{
                OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersion, RequestState,
            },
        },
    },
};

pub use crate::devices::soundcore::common::modules::button_configuration_v2::COMMON_SETTINGS as BUTTON_CONFIGURATION_SETTINGS;

mod packets;
mod state;

soundcore_device!(
    A3926State,
    A3926StateUpdatePacket,
    async |packet_io| {
        let state_update_packet: A3926StateUpdatePacket = packet_io
            .send_with_response(&RequestState::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersion = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersion::new().into())
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
        builder.button_configuration_v2(&BUTTON_CONFIGURATION_SETTINGS);
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestState::COMMAND,
                A3926StateUpdatePacket::default().bytes(),
            ),
            (
                RequestSerialNumberAndFirmwareVersion::COMMAND,
                SerialNumberAndFirmwareVersion::default().bytes(),
            ),
        ])
    },
);

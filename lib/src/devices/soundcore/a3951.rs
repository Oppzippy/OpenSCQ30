use std::collections::HashMap;

use crate::devices::soundcore::{
    a3951::{packets::A3951StateUpdatePacket, state::A3951State},
    common::{
        macros::soundcore_device,
        modules::{
            button_configuration::COMMON_SETTINGS as BUTTON_SETTINGS,
            sound_modes::AvailableSoundModes,
        },
        packet::{
            inbound::{SerialNumberAndFirmwareVersion, TryToPacket},
            outbound::{RequestSerialNumberAndFirmwareVersion, RequestState, ToPacket},
        },
        structures::{AmbientSoundMode, NoiseCancelingMode, TransparencyMode},
    },
};

mod packets;
mod state;

soundcore_device!(
    A3951State,
    async |packet_io| {
        let state_update_packet: A3951StateUpdatePacket = packet_io
            .send_with_response(&RequestState::default().to_packet())
            .await?
            .try_to_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersion = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersion::default().to_packet())
            .await?
            .try_to_packet()?;
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
        builder.equalizer_with_custom_hear_id_tws().await;
        builder.button_configuration(&BUTTON_SETTINGS);
        builder.reset_button_configuration::<A3951StateUpdatePacket>(
            RequestState::default().to_packet(),
        );
        builder.touch_tone();
        builder.wearing_detection();
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestState::COMMAND,
                A3951StateUpdatePacket::default().to_packet().bytes(),
            ),
            (
                RequestSerialNumberAndFirmwareVersion::COMMAND,
                SerialNumberAndFirmwareVersion::default()
                    .to_packet()
                    .bytes(),
            ),
        ])
    },
);

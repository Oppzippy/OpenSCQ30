use std::collections::HashMap;

use crate::devices::soundcore::{
    a3031::{packets::A3031StateUpdatePacket, state::A3031State},
    common::{
        self,
        macros::soundcore_device,
        modules::{
            button_configuration::COMMON_SETTINGS as BUTTON_CONFIGURATION_SETTINGS, equalizer,
            sound_modes::AvailableSoundModes,
        },
        packet::{
            inbound::{SerialNumberAndFirmwareVersion, TryToPacket},
            outbound::{RequestSerialNumberAndFirmwareVersion, RequestState, ToPacket},
        },
        structures::{AmbientSoundMode, NoiseCancelingMode},
    },
};

mod packets;
mod state;

soundcore_device!(
    A3031State,
    async |packet_io| {
        let state_update_packet: A3031StateUpdatePacket = packet_io
            .send_with_response(&RequestState::default().to_packet())
            .await?
            .try_to_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersion = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersion::default().to_packet())
            .await?
            .try_to_packet()?;
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
        builder.equalizer_tws(equalizer::common_settings()).await;
        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::half_hour_increments(),
        );
        builder.touch_tone();
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestState::COMMAND,
                A3031StateUpdatePacket::default().to_packet(),
            ),
            (
                RequestSerialNumberAndFirmwareVersion::COMMAND,
                SerialNumberAndFirmwareVersion::default().to_packet(),
            ),
        ])
    },
);

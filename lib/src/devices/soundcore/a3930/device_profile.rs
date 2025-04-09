use crate::devices::soundcore::standard::{
    macros::soundcore_device,
    modules::sound_modes::AvailableSoundModes,
    packets::{
        inbound::{SerialNumberAndFirmwareVersionUpdatePacket, TryIntoInboundPacket},
        outbound::{RequestFirmwareVersionPacket, RequestStatePacket},
    },
    structures::AmbientSoundMode,
};

use super::{packets::A3930StateUpdatePacket, state::A3930State};

soundcore_device!(
    A3930State,
    A3930StateUpdatePacket,
    async |packet_io| {
        let state_update_packet: A3930StateUpdatePacket = packet_io
            .send(&RequestStatePacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersionUpdatePacket = packet_io
            .send(&RequestFirmwareVersionPacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        Ok(A3930State::new(state_update_packet, sn_and_firmware))
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.sound_modes(AvailableSoundModes {
            ambient_sound_modes: vec![AmbientSoundMode::Normal, AmbientSoundMode::Transparency],
            transparency_modes: Vec::new(),
            noise_canceling_modes: Vec::new(),
        });
        builder.equalizer_with_custom_hear_id().await;
        builder.button_configuration();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    }
);

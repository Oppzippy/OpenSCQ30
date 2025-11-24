use std::collections::HashMap;

use crate::devices::soundcore::a3004::packets::A3004StateUpdatePacket;
use crate::devices::soundcore::a3004::state::A3004State;
use crate::devices::soundcore::common::modules::equalizer;
use crate::devices::soundcore::common::packet::outbound::ToPacket;
use crate::devices::soundcore::common::{
    device::fetch_state_from_state_update_packet, macros::soundcore_device,
    modules::sound_modes::AvailableSoundModes, packet::outbound::RequestState,
    structures::AmbientSoundMode,
};

mod packets;
mod state;

soundcore_device!(
    A3004State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3004State, A3004StateUpdatePacket>(packet_io)
            .await
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
            noise_canceling_modes: vec![],
        });
        builder
            .equalizer_with_drc(equalizer::common_settings())
            .await;
        builder.single_battery();
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3004StateUpdatePacket::default().to_packet(),
        )])
    },
);

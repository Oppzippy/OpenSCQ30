use std::collections::HashMap;

use crate::devices::soundcore::{
    a3027::{packets::A3027StateUpdatePacket, state::A3027State},
    common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        modules::{equalizer, sound_modes::AvailableSoundModes},
        packet::outbound::{RequestState, ToPacket},
        structures::{AmbientSoundMode, NoiseCancelingMode},
    },
};

mod packets;
mod state;

soundcore_device!(
    A3027State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3027State, A3027StateUpdatePacket>(packet_io)
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
            noise_canceling_modes: vec![
                NoiseCancelingMode::Transport,
                NoiseCancelingMode::Indoor,
                NoiseCancelingMode::Outdoor,
            ],
        });
        builder.equalizer(equalizer::common_settings()).await;
        builder.single_battery();
        builder.wearing_detection();
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3027StateUpdatePacket::default().to_packet(),
        )])
    },
);

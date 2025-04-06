use crate::devices::soundcore::standard::{
    macros::soundcore_device,
    modules::sound_modes::AvailableSoundModes,
    structures::{AmbientSoundMode, NoiseCancelingMode, TransparencyMode},
};

use super::{packets::A3951StateUpdatePacket, state::A3951State};

soundcore_device!(A3951State, A3951StateUpdatePacket, async |builder| {
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
});

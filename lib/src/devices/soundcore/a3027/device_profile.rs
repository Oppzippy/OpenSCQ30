use crate::devices::soundcore::standard::{
    macros::soundcore_device,
    modules::sound_modes::AvailableSoundModes,
    structures::{AmbientSoundMode, NoiseCancelingMode},
};

use super::{packets::A3027StateUpdatePacket, state::A3027State};

soundcore_device!(A3027State, A3027StateUpdatePacket, async |builder| {
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
});

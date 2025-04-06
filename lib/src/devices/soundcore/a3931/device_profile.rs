use crate::devices::soundcore::standard::{
    macros::soundcore_device,
    modules::sound_modes::AvailableSoundModes,
    structures::{AmbientSoundMode, TransparencyMode},
};

use super::{packets::A3931StateUpdatePacket, state::A3931State};

soundcore_device!(A3931State, A3931StateUpdatePacket, async |builder| {
    builder.module_collection().add_state_update();
    builder.sound_modes(AvailableSoundModes {
        ambient_sound_modes: vec![AmbientSoundMode::Normal, AmbientSoundMode::Transparency],
        transparency_modes: vec![
            TransparencyMode::FullyTransparent,
            TransparencyMode::VocalMode,
        ],
        noise_canceling_modes: Vec::new(),
    });
    builder.equalizer_with_drc().await;
    builder.button_configuration();
});

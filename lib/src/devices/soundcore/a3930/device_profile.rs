use crate::devices::soundcore::standard::{
    macros::soundcore_device, modules::sound_modes::AvailableSoundModes,
    structures::AmbientSoundMode,
};

use super::{packets::A3930StateUpdatePacket, state::A3930State};

soundcore_device!(A3930State, A3930StateUpdatePacket, async |builder| {
    builder.module_collection().add_state_update();
    builder.sound_modes(AvailableSoundModes {
        ambient_sound_modes: vec![AmbientSoundMode::Normal, AmbientSoundMode::Transparency],
        transparency_modes: Vec::new(),
        noise_canceling_modes: Vec::new(),
    });
    builder.stereo_equalizer_with_custom_hear_id().await;
    builder.button_configuration();
});

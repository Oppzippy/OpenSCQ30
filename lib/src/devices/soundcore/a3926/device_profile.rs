use crate::devices::soundcore::standard::macros::soundcore_device;

use super::{packets::A3926StateUpdatePacket, state::A3926State};

soundcore_device!(A3926State, A3926StateUpdatePacket, async |builder| {
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
    builder.stereo_equalizer_with_basic_hear_id().await;
    builder.button_configuration();
});

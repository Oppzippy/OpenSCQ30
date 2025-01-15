use crate::{
    devices::standard::{
        packets::outbound::SetSoundModePacket,
        state::DeviceState,
        structures::{AmbientSoundMode, SoundModes},
    },
    soundcore_device::device::soundcore_command::CommandResponse,
};

pub fn set_sound_modes(
    state: DeviceState,
    sound_modes: SoundModes,
) -> crate::Result<CommandResponse> {
    let Some(prev_sound_modes) = state.sound_modes else {
        return Err(crate::Error::MissingData {
            name: "sound modes",
        });
    };

    // It will bug and put us in noise canceling mode without changing the ambient sound mode id if we change the
    // noise canceling mode with the ambient sound mode being normal or transparency. To work around this, we must
    // set the ambient sound mode to Noise Canceling, and then change it back.
    let needs_noise_canceling = prev_sound_modes.ambient_sound_mode
        != AmbientSoundMode::NoiseCanceling
        && prev_sound_modes.noise_canceling_mode != sound_modes.noise_canceling_mode;
    let needs_ambient_sound_mode_revert =
        needs_noise_canceling && sound_modes.ambient_sound_mode != AmbientSoundMode::NoiseCanceling;
    let mut packets = Vec::with_capacity(
        usize::from(needs_ambient_sound_mode_revert)
            + 1
            + usize::from(needs_ambient_sound_mode_revert),
    );

    if needs_noise_canceling {
        packets.push(
            SetSoundModePacket(SoundModes {
                ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
                noise_canceling_mode: prev_sound_modes.noise_canceling_mode,
                transparency_mode: prev_sound_modes.transparency_mode,
                custom_noise_canceling: prev_sound_modes.custom_noise_canceling,
            })
            .into(),
        );
    }

    // If we need to temporarily be in noise canceling mode to work around the bug, set all fields besides
    // ambient_sound_mode. Otherwise, we set all fields in one go.
    packets.push(
        SetSoundModePacket(SoundModes {
            ambient_sound_mode: if needs_noise_canceling {
                AmbientSoundMode::NoiseCanceling
            } else {
                sound_modes.ambient_sound_mode
            },
            noise_canceling_mode: sound_modes.noise_canceling_mode,
            transparency_mode: sound_modes.transparency_mode,
            custom_noise_canceling: sound_modes.custom_noise_canceling,
        })
        .into(),
    );

    // Switch to the target sound mode if we didn't do it in the previous step.
    // If the target sound mode is noise canceling, we already set it to that, so no change needed.
    if needs_ambient_sound_mode_revert {
        packets.push(
            SetSoundModePacket(SoundModes {
                ambient_sound_mode: sound_modes.ambient_sound_mode,
                noise_canceling_mode: sound_modes.noise_canceling_mode,
                transparency_mode: sound_modes.transparency_mode,
                custom_noise_canceling: sound_modes.custom_noise_canceling,
            })
            .into(),
        );
    }

    Ok(CommandResponse {
        packets,
        new_state: DeviceState {
            sound_modes: Some(sound_modes),
            ..state
        },
    })
}

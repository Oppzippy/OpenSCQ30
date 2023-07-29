use openscq30_lib::{
    api::device::Device,
    packets::structures::{EqualizerConfiguration, SoundModes, VolumeAdjustments},
};

use crate::cli::SetCommand;

pub async fn set(set_command: SetCommand, device: &impl Device) -> openscq30_lib::Result<()> {
    let device_state = device.state().await;
    match set_command {
        SetCommand::AmbientSoundMode { mode } => {
            if let Some(sound_modes) = device_state.sound_modes {
                device
                    .set_sound_modes(SoundModes {
                        ambient_sound_mode: mode.into(),
                        ..sound_modes
                    })
                    .await?
            }
        }
        SetCommand::NoiseCancelingMode { mode } => {
            if let Some(sound_modes) = device_state.sound_modes {
                device
                    .set_sound_modes(SoundModes {
                        noise_canceling_mode: mode.into(),
                        ..sound_modes
                    })
                    .await?
            }
        }
        SetCommand::Equalizer { volume_adjustments } => {
            let volume_adjustments = volume_adjustments
                .try_into()
                .map(VolumeAdjustments::new)
                .unwrap_or_else(|values| {
                    panic!(
                        "error converting vec of volume adjustments to array: expected len 8, got {}",
                        values.len()
                    )
                });

            device
                .set_equalizer_configuration(EqualizerConfiguration::new_custom_profile(
                    volume_adjustments,
                ))
                .await?
        }
    };
    Ok(())
}

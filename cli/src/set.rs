use openscq30_lib::{
    api::device::Device,
    devices::standard::structures::{EqualizerConfiguration, SoundModes, VolumeAdjustments},
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
            let adjustment_array: [i8; 8] =
                volume_adjustments
                    .try_into()
                    .unwrap_or_else(|values: Vec<_>| {
                        panic!(
                            "error converting vec of volume adjustments to array: expected len 8, got {}",
                            values.len(),
                        )
                    });
            let float_adjustments = adjustment_array.map(|adjustment| (adjustment as f64) / 10.0);

            device
                .set_equalizer_configuration(EqualizerConfiguration::new_custom_profile(
                    VolumeAdjustments::new(float_adjustments)
                        .expect("we already checked for valid length"),
                ))
                .await?
        }
    };
    Ok(())
}

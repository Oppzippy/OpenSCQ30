use anyhow::bail;
use openscq30_lib::{
    api::device::Device,
    devices::standard::structures::{
        EqualizerConfiguration, SoundModes, SoundModesTypeTwo, VolumeAdjustments,
    },
};

use crate::{args::NoiseCancelingModeKind, cli::SetCommand};

pub async fn set(set_command: SetCommand, device: &impl Device) -> anyhow::Result<()> {
    let device_state = device.state().await;
    match set_command {
        SetCommand::AmbientSoundMode { mode } => {
            match (device_state.sound_modes, device_state.sound_modes_type_two) {
                (Some(sound_modes), _) => {
                    device
                        .set_sound_modes(SoundModes {
                            ambient_sound_mode: mode.into(),
                            ..sound_modes
                        })
                        .await?
                }
                (None, Some(sound_modes_type_two)) => {
                    device
                        .set_sound_modes_type_two(SoundModesTypeTwo {
                            ambient_sound_mode: mode.into(),
                            ..sound_modes_type_two
                        })
                        .await?
                }
                (None, None) => bail!("sound modes not supported by the device"),
            }
        }
        SetCommand::TransparencyMode { mode } => {
            match (device_state.sound_modes, device_state.sound_modes_type_two) {
                (Some(sound_modes), _) => {
                    device
                        .set_sound_modes(SoundModes {
                            transparency_mode: mode.into(),
                            ..sound_modes
                        })
                        .await?
                }
                (None, Some(sound_modes_type_two)) => {
                    device
                        .set_sound_modes_type_two(SoundModesTypeTwo {
                            transparency_mode: mode.into(),
                            ..sound_modes_type_two
                        })
                        .await?
                }
                (None, None) => bail!("sound modes not supported by the device"),
            }
        }
        SetCommand::NoiseCancelingMode { mode } => match mode.kind() {
            NoiseCancelingModeKind::TypeOne => {
                let Some(sound_modes) = device_state.sound_modes else {
                    bail!("this noise canceling mode is not supported by the device")
                };
                device
                    .set_sound_modes(SoundModes {
                        noise_canceling_mode: mode.try_into()?,
                        ..sound_modes
                    })
                    .await?
            }
            NoiseCancelingModeKind::TypeTwo => {
                let Some(sound_modes) = device_state.sound_modes_type_two else {
                    bail!("adaptive/manual noise canceling not supported by device");
                };
                device
                    .set_sound_modes_type_two(SoundModesTypeTwo {
                        noise_canceling_mode: mode.try_into()?,
                        ..sound_modes
                    })
                    .await?;
            }
        },
        SetCommand::ManualNoiseCanceling { mode } => {
            let Some(sound_modes) = device_state.sound_modes_type_two else {
                bail!("adaptive noise canceling not supported by device");
            };
            device
                .set_sound_modes_type_two(SoundModesTypeTwo {
                    manual_noise_canceling: mode.into(),
                    ..sound_modes
                })
                .await?;
        }
        SetCommand::Equalizer { volume_adjustments } => {
            let adjustment_array: [i16; 8] =
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
                    VolumeAdjustments::new(float_adjustments)?,
                ))
                .await?
        }
    };
    Ok(())
}

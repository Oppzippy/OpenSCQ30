use openscq30_lib::{
    api::device::Device,
    packets::structures::{EqualizerBandOffsets, EqualizerConfiguration},
};

use crate::cli::SetCommand;

pub async fn set(set_command: SetCommand, device: &impl Device) -> openscq30_lib::Result<()> {
    match set_command {
        SetCommand::AmbientSoundMode { mode } => device.set_ambient_sound_mode(mode.into()).await?,
        SetCommand::NoiseCancelingMode { mode } => {
            device.set_noise_canceling_mode(mode.into()).await?
        }
        SetCommand::Equalizer { band_offsets } => {
            let band_offsets = band_offsets
                .try_into()
                .map(EqualizerBandOffsets::new)
                .unwrap_or_else(|values| {
                    panic!(
                        "error converting vec of band offsets to array: expected len 8, got {}",
                        values.len()
                    )
                });

            device
                .set_equalizer_configuration(EqualizerConfiguration::new_custom_profile(
                    band_offsets,
                ))
                .await?
        }
    };
    Ok(())
}

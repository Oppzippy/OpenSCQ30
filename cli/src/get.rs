use anyhow::bail;
use heck::AsKebabCase;
use itertools::Itertools;
use openscq30_lib::{api::device::Device, devices::standard::structures::VolumeAdjustments};

use crate::cli::GetCommand;

pub async fn get(get_command: GetCommand, device: &impl Device) -> anyhow::Result<()> {
    let device_state = device.state().await;
    match get_command {
        GetCommand::AmbientSoundMode => {
            let ambient_sound_mode: &'static str =
                match (device_state.sound_modes, device_state.sound_modes_type_two) {
                    (Some(sound_modes), _) => sound_modes.ambient_sound_mode.into(),
                    (None, Some(sound_modes_type_two)) => {
                        sound_modes_type_two.ambient_sound_mode.into()
                    }
                    (None, None) => bail!("sound modes not supported by the device"),
                };
            println!("{}", AsKebabCase(ambient_sound_mode))
        }
        GetCommand::TransparencyMode => {
            let transparency_mode: &'static str =
                match (device_state.sound_modes, device_state.sound_modes_type_two) {
                    (Some(sound_modes), _) => sound_modes.transparency_mode.into(),
                    (None, Some(sound_modes_type_two)) => {
                        sound_modes_type_two.transparency_mode.into()
                    }
                    (None, None) => bail!("sound modes not supported by the device"),
                };
            println!("{}", AsKebabCase(transparency_mode))
        }
        GetCommand::NoiseCancelingMode => {
            let noise_canceling_mode: &'static str =
                match (device_state.sound_modes, device_state.sound_modes_type_two) {
                    (Some(sound_modes), _) => sound_modes.noise_canceling_mode.into(),
                    (None, Some(sound_modes_type_two)) => {
                        sound_modes_type_two.noise_canceling_mode.into()
                    }
                    (None, None) => bail!("sound modes not supported by the device"),
                };
            println!("{}", AsKebabCase(noise_canceling_mode))
        }
        GetCommand::AdaptiveNoiseCanceling => {
            let Some(sound_modes) = device_state.sound_modes_type_two else {
                bail!("adaptive noise canceling not supported by device");
            };
            println!(
                "{}",
                AsKebabCase(sound_modes.adaptive_noise_canceling.as_ref())
            );
        }
        GetCommand::ManualNoiseCanceling => {
            let Some(sound_modes) = device_state.sound_modes_type_two else {
                bail!("manual noise canceling not supported by device");
            };
            println!(
                "{}",
                AsKebabCase(sound_modes.manual_noise_canceling.as_ref())
            );
        }
        GetCommand::Equalizer => {
            print_volume_adjustments(device_state.equalizer_configuration.volume_adjustments())
        }
    };
    Ok(())
}

fn print_volume_adjustments(volume_adjustments: &VolumeAdjustments) {
    let separated_volume_adjustments = volume_adjustments
        .adjustments()
        .iter()
        .cloned()
        .map(|adjustment| format!("{:.0}", adjustment * 10.0))
        .join(" ");
    println!("{separated_volume_adjustments}");
}

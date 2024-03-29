use heck::AsKebabCase;
use itertools::Itertools;
use openscq30_lib::{api::device::Device, devices::standard::structures::VolumeAdjustments};

use crate::cli::GetCommand;

pub async fn get(get_command: GetCommand, device: &impl Device) {
    let device_state = device.state().await;
    match get_command {
        GetCommand::AmbientSoundMode => {
            if let Some(sound_modes) = device_state.sound_modes {
                let cli_case = AsKebabCase(sound_modes.ambient_sound_mode.to_string());
                println!("{}", cli_case)
            }
        }
        GetCommand::NoiseCancelingMode => {
            if let Some(sound_modes) = device_state.sound_modes {
                let cli_case = AsKebabCase(sound_modes.noise_canceling_mode.to_string());
                println!("{}", cli_case)
            }
        }
        GetCommand::Equalizer => {
            print_volume_adjustments(device_state.equalizer_configuration.volume_adjustments())
        }
    };
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

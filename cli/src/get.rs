use heck::AsKebabCase;
use openscq30_lib::api::device::Device;

use crate::cli::GetCommand;

pub async fn get(get_command: GetCommand, device: &impl Device) {
    match get_command {
        GetCommand::AmbientSoundMode => {
            let mode = device.ambient_sound_mode().await;
            let cli_case = AsKebabCase(mode.to_string());
            println!("{}", cli_case)
        }
        GetCommand::NoiseCancelingMode => {
            let mode = device.noise_canceling_mode().await;
            let cli_case = AsKebabCase(mode.to_string());
            println!("{}", cli_case)
        }
        GetCommand::Equalizer => {
            let equalizer_configuration = device.equalizer_configuration().await;
            let separated_volume_adjustments = equalizer_configuration
                .volume_adjustments()
                .bytes()
                .map(|b| b.to_string())
                .join(" ");
            println!("{separated_volume_adjustments}");
        }
    };
}

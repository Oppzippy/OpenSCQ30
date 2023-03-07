use openscq30_lib::api::device::Device;

use crate::cli::GetCommand;

pub async fn get(get_command: GetCommand, device: &impl Device) {
    match get_command {
        GetCommand::AmbientSoundMode => {
            println!("{}", device.ambient_sound_mode().await.to_string())
        }
        GetCommand::NoiseCancelingMode => {
            println!("{}", device.noise_canceling_mode().await.to_string())
        }
        GetCommand::Equalizer => {
            let equalizer_configuration = device.equalizer_configuration().await;
            let separated_band_offsets = equalizer_configuration
                .band_offsets()
                .bytes()
                .map(|b| b.to_string())
                .join(" ");
            println!("{separated_band_offsets}");
        }
    };
}

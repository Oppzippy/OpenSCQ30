use std::error::Error;

use clap::{command, Parser, Subcommand, ValueEnum};
use openscq30_lib::{
    api::soundcore_device_registry::SoundcoreDeviceRegistry,
    packets::structures::{
        equalizer_band_offsets::EqualizerBandOffsets,
        equalizer_configuration::EqualizerConfiguration,
    },
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(subcommand)]
    Set(SetCommand),
    #[command(subcommand)]
    Get(GetCommand),
}

#[derive(Subcommand)]
enum SetCommand {
    AmbientSoundMode {
        #[arg(value_enum)]
        mode: AmbientSoundMode,
    },
    NoiseCancelingMode {
        #[arg(value_enum)]
        mode: NoiseCancelingMode,
    },
    Equalizer {
        #[arg(required=true, num_args = 8, value_parser = clap::value_parser!(i8).range(-60..60))]
        band_values: Vec<i8>,
    },
}

#[derive(Subcommand)]
enum GetCommand {
    AmbientSoundMode,
    NoiseCancelingMode,
    Equalizer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum AmbientSoundMode {
    Normal,
    Transparency,
    NoiseCanceling,
}

impl AmbientSoundMode {
    pub fn to_packet_structure(
        &self,
    ) -> openscq30_lib::packets::structures::ambient_sound_mode::AmbientSoundMode {
        match self {
            AmbientSoundMode::Normal => openscq30_lib::packets::structures::ambient_sound_mode::AmbientSoundMode::Normal,
            AmbientSoundMode::Transparency => openscq30_lib::packets::structures::ambient_sound_mode::AmbientSoundMode::Transparency,
            AmbientSoundMode::NoiseCanceling => openscq30_lib::packets::structures::ambient_sound_mode::AmbientSoundMode::NoiseCanceling,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum NoiseCancelingMode {
    Transport,
    Indoor,
    Outdoor,
}

impl NoiseCancelingMode {
    pub fn to_packet_structure(
        &self,
    ) -> openscq30_lib::packets::structures::noise_canceling_mode::NoiseCancelingMode {
        match self {
            NoiseCancelingMode::Transport => openscq30_lib::packets::structures::noise_canceling_mode::NoiseCancelingMode::Transport,
            NoiseCancelingMode::Indoor => openscq30_lib::packets::structures::noise_canceling_mode::NoiseCancelingMode::Indoor,
            NoiseCancelingMode::Outdoor => openscq30_lib::packets::structures::noise_canceling_mode::NoiseCancelingMode::Outdoor,
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();
    let mut registry = SoundcoreDeviceRegistry::new().await?;
    registry.refresh_devices().await?;

    let devices = registry.get_devices().await;
    if let Some(device) = devices.first() {
        match args.command {
            Command::Set(set_command) => match set_command {
                SetCommand::AmbientSoundMode { mode } => {
                    device
                        .set_ambient_sound_mode(mode.to_packet_structure())
                        .await?
                }
                SetCommand::NoiseCancelingMode { mode } => {
                    device
                        .set_noise_canceling_mode(mode.to_packet_structure())
                        .await?
                }
                SetCommand::Equalizer { band_values } => {
                    device
                        .set_equalizer_configuration(EqualizerConfiguration::Custom(
                            EqualizerBandOffsets::new([
                                band_values[0],
                                band_values[1],
                                band_values[2],
                                band_values[3],
                                band_values[4],
                                band_values[5],
                                band_values[6],
                                band_values[7],
                            ]),
                        ))
                        .await?
                }
            },
            Command::Get(get_command) => match get_command {
                GetCommand::AmbientSoundMode => {
                    println!("{}", device.get_ambient_sound_mode().await.id())
                }
                GetCommand::NoiseCancelingMode => {
                    println!("{}", device.get_noise_canceling_mode().await.id())
                }
                GetCommand::Equalizer => {
                    let equalizer_configuration = device.get_equalizer_configuration().await;
                    let band_offsets = equalizer_configuration.band_offsets().volume_offsets();
                    println!(
                        "{} [{} {} {} {} {} {} {} {}]",
                        equalizer_configuration.profile_id(),
                        band_offsets[0],
                        band_offsets[1],
                        band_offsets[2],
                        band_offsets[3],
                        band_offsets[4],
                        band_offsets[5],
                        band_offsets[6],
                        band_offsets[7],
                    );
                }
            },
        };
    } else {
        println!("Not connected to headphones.");
    }
    Ok(())
}

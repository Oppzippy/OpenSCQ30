use std::error::Error;

use clap::{command, Parser, Subcommand, ValueEnum};
use openscq30_lib::{
    api::SoundcoreDeviceRegistry,
    packets::structures::{EqualizerBandOffsets, EqualizerConfiguration},
    soundcore_bluetooth::btleplug,
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
        band_offsets: Vec<i8>,
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

impl From<AmbientSoundMode> for openscq30_lib::packets::structures::AmbientSoundMode {
    fn from(mode: AmbientSoundMode) -> Self {
        match mode {
            AmbientSoundMode::Normal => {
                openscq30_lib::packets::structures::AmbientSoundMode::Normal
            }
            AmbientSoundMode::Transparency => {
                openscq30_lib::packets::structures::AmbientSoundMode::Transparency
            }
            AmbientSoundMode::NoiseCanceling => {
                openscq30_lib::packets::structures::AmbientSoundMode::NoiseCanceling
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum NoiseCancelingMode {
    Transport,
    Indoor,
    Outdoor,
}

impl From<NoiseCancelingMode> for openscq30_lib::packets::structures::NoiseCancelingMode {
    fn from(mode: NoiseCancelingMode) -> Self {
        match mode {
            NoiseCancelingMode::Transport => {
                openscq30_lib::packets::structures::NoiseCancelingMode::Transport
            }
            NoiseCancelingMode::Indoor => {
                openscq30_lib::packets::structures::NoiseCancelingMode::Indoor
            }
            NoiseCancelingMode::Outdoor => {
                openscq30_lib::packets::structures::NoiseCancelingMode::Outdoor
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .init();

    let args = Cli::parse();
    let connection_registry_impl = btleplug::new_connection_registry()
        .await
        .unwrap_or_else(|err| panic!("failed to initialize handler: {err}"));
    let registry = SoundcoreDeviceRegistry::new(connection_registry_impl).await?;
    registry.refresh_devices().await?;

    let devices = registry.devices().await;
    if let Some(device) = devices.first() {
        match args.command {
            Command::Set(set_command) => match set_command {
                SetCommand::AmbientSoundMode { mode } => {
                    device.set_ambient_sound_mode(mode.into()).await?
                }
                SetCommand::NoiseCancelingMode { mode } => {
                    device.set_noise_canceling_mode(mode.into()).await?
                }
                SetCommand::Equalizer { band_offsets } => {
                    let band_offsets = band_offsets
                        .try_into()
                        .map(EqualizerBandOffsets::new)
                        .unwrap_or_else(|values| {
                            panic!("error converting vec of band offsets to array: expected len 8, got {}", values.len())
                        });

                    device
                        .set_equalizer_configuration(EqualizerConfiguration::new_custom_profile(
                            band_offsets,
                        ))
                        .await?
                }
            },
            Command::Get(get_command) => match get_command {
                GetCommand::AmbientSoundMode => {
                    println!("{}", device.ambient_sound_mode().await.id())
                }
                GetCommand::NoiseCancelingMode => {
                    println!("{}", device.noise_canceling_mode().await.id())
                }
                GetCommand::Equalizer => {
                    let equalizer_configuration = device.equalizer_configuration().await;
                    println!("{:?}", equalizer_configuration);
                }
            },
        };
    } else {
        println!("Not connected to headphones.");
    }
    Ok(())
}

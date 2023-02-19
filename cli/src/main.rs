use std::error::Error;

use clap::{command, Parser, Subcommand, ValueEnum};
use openscq30_lib::{
    api::device::{SoundcoreDevice, SoundcoreDeviceDescriptor, SoundcoreDeviceRegistry},
    packets::structures::{EqualizerBandOffsets, EqualizerConfiguration},
};
use tracing::Level;
#[cfg(debug_assertions)]
use tracing_subscriber::fmt::format::FmtSpan;

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
    let subscriber_builder = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .pretty();
    #[cfg(debug_assertions)]
    let subscriber_builder = subscriber_builder
        .with_max_level(Level::TRACE)
        .with_span_events(FmtSpan::ACTIVE);
    #[cfg(not(debug_assertions))]
    let subscriber_builder = subscriber_builder.with_max_level(Level::INFO);
    subscriber_builder.init();

    let args = Cli::parse();
    let registry = openscq30_lib::api::new_soundcore_device_registry()
        .await
        .unwrap_or_else(|err| panic!("failed to initialize device registry: {err}"));
    do_cli_command(args, registry).await
}

// rust-analyzer doesn't seem to work with the associated types of an impl Trait return value
// as a workaround, we can immediately pass the return value as a parameter to another function
async fn do_cli_command(
    args: Cli,
    registry: impl SoundcoreDeviceRegistry,
) -> Result<(), Box<dyn Error>> {
    let descriptors = registry.device_descriptors().await?;
    let first = descriptors.first().unwrap();
    let device = registry.device(first.mac_address()).await?;

    if let Some(device) = device {
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
                    println!("{}", device.ambient_sound_mode().await.to_string())
                }
                GetCommand::NoiseCancelingMode => {
                    println!("{}", device.noise_canceling_mode().await.to_string())
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

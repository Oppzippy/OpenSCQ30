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

impl From<AmbientSoundMode>
    for openscq30_lib::packets::structures::ambient_sound_mode::AmbientSoundMode
{
    fn from(mode: AmbientSoundMode) -> Self {
        match mode {
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

impl From<NoiseCancelingMode>
    for openscq30_lib::packets::structures::noise_canceling_mode::NoiseCancelingMode
{
    fn from(mode: NoiseCancelingMode) -> Self {
        match mode {
            NoiseCancelingMode::Transport => openscq30_lib::packets::structures::noise_canceling_mode::NoiseCancelingMode::Transport,
            NoiseCancelingMode::Indoor => openscq30_lib::packets::structures::noise_canceling_mode::NoiseCancelingMode::Indoor,
            NoiseCancelingMode::Outdoor => openscq30_lib::packets::structures::noise_canceling_mode::NoiseCancelingMode::Outdoor,
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
    let registry = SoundcoreDeviceRegistry::new().await?;
    registry.refresh_devices().await?;

    let devices = registry.get_devices().await;
    if let Some(device) = devices.first() {
        match args.command {
            Command::Set(set_command) => match set_command {
                SetCommand::AmbientSoundMode { mode } => {
                    device.set_ambient_sound_mode(mode.into()).await?
                }
                SetCommand::NoiseCancelingMode { mode } => {
                    device.set_noise_canceling_mode(mode.into()).await?
                }
                SetCommand::Equalizer { band_values } => {
                    device
                        .set_equalizer_configuration(EqualizerConfiguration::new_custom_profile(
                            EqualizerBandOffsets::new(band_values.try_into().unwrap()),
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
                    println!("{:?}", equalizer_configuration);
                }
            },
        };
    } else {
        println!("Not connected to headphones.");
    }
    Ok(())
}

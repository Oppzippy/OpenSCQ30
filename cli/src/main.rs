use std::{error::Error, sync::Arc};

use clap::{command, Parser, Subcommand, ValueEnum};
use macaddr::MacAddr6;
use openscq30_lib::{
    api::device::{Device, DeviceDescriptor, DeviceRegistry},
    packets::structures::{EqualizerBandOffsets, EqualizerConfiguration},
};
use tracing::Level;
#[cfg(debug_assertions)]
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    mac_address: Option<MacAddr6>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(subcommand)]
    Set(SetCommand),
    #[command(subcommand)]
    Get(GetCommand),
    ListDevices,
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
async fn do_cli_command(args: Cli, registry: impl DeviceRegistry) -> Result<(), Box<dyn Error>> {
    let descriptors = registry.device_descriptors().await?;
    let selected_descriptor = args
        .mac_address
        .map(|mac_address| {
            let mac_address_string = mac_address.to_string();
            descriptors
                .iter()
                .find(|descriptor| descriptor.mac_address() == &mac_address_string)
        })
        .or_else(|| Some(descriptors.first()))
        .flatten();

    match (args.command, selected_descriptor) {
        (Command::ListDevices, _) => list_devices(&descriptors),
        (Command::Set(set_command), Some(descriptor)) => {
            let device = get_device_or_err(&registry, descriptor).await?;
            set(set_command, device.as_ref()).await?;
        }
        (Command::Get(get_command), Some(descriptor)) => {
            let device = get_device_or_err(&registry, descriptor).await?;
            get(get_command, device.as_ref()).await;
        }
        (_, None) => println!("No device found."),
    };
    Ok(())
}

async fn get_device_or_err<T>(
    registry: &T,
    descriptor: &T::DescriptorType,
) -> Result<Arc<T::DeviceType>, String>
where
    T: DeviceRegistry,
{
    match registry.device(descriptor.mac_address()).await {
        Ok(Some(device)) => Ok(device),
        Err(err) => Err(format!("Error fetching device: {err}")),
        Ok(None) => Err("No device found.".to_string()),
    }
}

async fn get(get_command: GetCommand, device: &impl Device) {
    match get_command {
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
    };
}

async fn set(set_command: SetCommand, device: &impl Device) -> openscq30_lib::Result<()> {
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

fn list_devices(descriptors: &[impl DeviceDescriptor]) {
    println!(
        "{}",
        descriptors
            .iter()
            .map(|descriptor| descriptor.mac_address().to_owned())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

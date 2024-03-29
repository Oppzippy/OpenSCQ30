use clap::{command, Parser, Subcommand, ValueEnum};
use macaddr::MacAddr6;
use openscq30_lib::devices::standard::structures::VolumeAdjustments;
use tracing::Level;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub mac_address: Option<MacAddr6>,
    #[arg(short, long, default_value_t = Level::WARN)]
    pub logging_level: Level,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Set(SetCommand),
    #[command(subcommand)]
    Get(GetCommand),
    ListDevices,
}

#[derive(Subcommand)]
pub enum SetCommand {
    AmbientSoundMode {
        #[arg(value_enum)]
        mode: AmbientSoundMode,
    },
    NoiseCancelingMode {
        #[arg(value_enum)]
        mode: NoiseCancelingMode,
    },
    Equalizer {
        #[arg(
            required=true,
            num_args = 8,
            value_parser = clap::value_parser!(i16).range((VolumeAdjustments::MIN_VOLUME * 10.0).round() as i64..(VolumeAdjustments::MAX_VOLUME * 10.0).round() as i64 + 1),
        )]
        volume_adjustments: Vec<i16>,
    },
}

#[derive(Subcommand)]
pub enum GetCommand {
    AmbientSoundMode,
    NoiseCancelingMode,
    Equalizer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AmbientSoundMode {
    Normal,
    Transparency,
    NoiseCanceling,
}

impl From<AmbientSoundMode> for openscq30_lib::devices::standard::structures::AmbientSoundMode {
    fn from(mode: AmbientSoundMode) -> Self {
        match mode {
            AmbientSoundMode::Normal => {
                openscq30_lib::devices::standard::structures::AmbientSoundMode::Normal
            }
            AmbientSoundMode::Transparency => {
                openscq30_lib::devices::standard::structures::AmbientSoundMode::Transparency
            }
            AmbientSoundMode::NoiseCanceling => {
                openscq30_lib::devices::standard::structures::AmbientSoundMode::NoiseCanceling
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum NoiseCancelingMode {
    Transport,
    Indoor,
    Outdoor,
}

impl From<NoiseCancelingMode> for openscq30_lib::devices::standard::structures::NoiseCancelingMode {
    fn from(mode: NoiseCancelingMode) -> Self {
        match mode {
            NoiseCancelingMode::Transport => {
                openscq30_lib::devices::standard::structures::NoiseCancelingMode::Transport
            }
            NoiseCancelingMode::Indoor => {
                openscq30_lib::devices::standard::structures::NoiseCancelingMode::Indoor
            }
            NoiseCancelingMode::Outdoor => {
                openscq30_lib::devices::standard::structures::NoiseCancelingMode::Outdoor
            }
        }
    }
}

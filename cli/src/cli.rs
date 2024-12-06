use crate::args::*;
use clap::{command, Parser, Subcommand, ValueEnum};
use macaddr::MacAddr6;
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
    Completions {
        #[arg(required = true)]
        shell: Shell,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[non_exhaustive]
pub enum Shell {
    Bash,
    Elvish,
    Fish,
    Powershell,
    Zsh,
}

impl From<Shell> for clap_complete::Shell {
    fn from(value: Shell) -> Self {
        match value {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Elvish => clap_complete::Shell::Elvish,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::Powershell => clap_complete::Shell::PowerShell,
            Shell::Zsh => clap_complete::Shell::Zsh,
        }
    }
}

#[derive(Subcommand)]
pub enum SetCommand {
    AmbientSoundMode {
        #[arg(value_enum)]
        mode: AmbientSoundMode,
    },
    TransparencyMode {
        #[arg(value_enum)]
        mode: TransparencyMode,
    },
    NoiseCancelingMode {
        #[arg(value_enum)]
        mode: NoiseCancelingMode,
    },
    #[command(about = "Only meaningful if noise-canceling-mode is manual")]
    ManualNoiseCanceling {
        #[arg(value_enum)]
        mode: ManualNoiseCanceling,
    },
    Equalizer {
        #[arg(
            required=true,
            num_args = 8,
            value_parser = clap::value_parser!(i16).range(VolumeAdjustments::range()),
        )]
        volume_adjustments: Vec<i16>,
    },
}

#[derive(Subcommand)]
pub enum GetCommand {
    AmbientSoundMode,
    TransparencyMode,
    NoiseCancelingMode,
    #[command(about = "Only meaningful if noise-canceling-mode is adaptive")]
    AdaptiveNoiseCanceling,
    #[command(about = "Only meaningful if noise-canceling-mode is manual")]
    ManualNoiseCanceling,
    Equalizer,
}

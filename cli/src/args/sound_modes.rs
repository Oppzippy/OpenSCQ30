use anyhow::anyhow;
use clap::ValueEnum;
use openscq30_lib::devices::standard::structures::{
    AdaptiveNoiseCanceling as LibAdaptiveNoiseCanceling, AmbientSoundMode as LibAmbientSoundMode,
    ManualNoiseCanceling as LibManualNoiseCancelingMode,
    NoiseCancelingMode as LibNoiseCancelingMode,
    NoiseCancelingModeTypeTwo as LibNoiseCancelingModeTypeTwo,
    TransparencyMode as LibTransparencyMode,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AmbientSoundMode {
    Normal,
    Transparency,
    NoiseCanceling,
}

impl From<AmbientSoundMode> for LibAmbientSoundMode {
    fn from(mode: AmbientSoundMode) -> Self {
        match mode {
            AmbientSoundMode::Normal => Self::Normal,
            AmbientSoundMode::Transparency => Self::Transparency,
            AmbientSoundMode::NoiseCanceling => Self::NoiseCanceling,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TransparencyMode {
    FullyTransparent,
    VocalMode,
}

impl From<TransparencyMode> for LibTransparencyMode {
    fn from(value: TransparencyMode) -> Self {
        match value {
            TransparencyMode::FullyTransparent => Self::FullyTransparent,
            TransparencyMode::VocalMode => Self::VocalMode,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum NoiseCancelingMode {
    Transport,
    Indoor,
    Outdoor,
    Custom,
    Adaptive,
    Manual,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum NoiseCancelingModeKind {
    TypeOne,
    TypeTwo,
}

impl NoiseCancelingMode {
    pub fn kind(&self) -> NoiseCancelingModeKind {
        match self {
            NoiseCancelingMode::Transport
            | NoiseCancelingMode::Indoor
            | NoiseCancelingMode::Outdoor
            | NoiseCancelingMode::Custom => NoiseCancelingModeKind::TypeOne,
            NoiseCancelingMode::Adaptive | NoiseCancelingMode::Manual => {
                NoiseCancelingModeKind::TypeTwo
            }
        }
    }
}

impl TryFrom<NoiseCancelingMode> for LibNoiseCancelingMode {
    type Error = anyhow::Error;

    fn try_from(mode: NoiseCancelingMode) -> Result<Self, Self::Error> {
        match mode {
            NoiseCancelingMode::Transport => Ok(Self::Transport),
            NoiseCancelingMode::Indoor => Ok(Self::Indoor),
            NoiseCancelingMode::Outdoor => Ok(Self::Outdoor),
            NoiseCancelingMode::Custom => Ok(Self::Custom),
            NoiseCancelingMode::Adaptive | NoiseCancelingMode::Manual => {
                Err(anyhow!("not a noise canceling mode type two"))
            }
        }
    }
}

impl TryFrom<NoiseCancelingMode> for LibNoiseCancelingModeTypeTwo {
    type Error = anyhow::Error;

    fn try_from(mode: NoiseCancelingMode) -> Result<Self, Self::Error> {
        match mode {
            NoiseCancelingMode::Transport
            | NoiseCancelingMode::Indoor
            | NoiseCancelingMode::Outdoor
            | NoiseCancelingMode::Custom => Err(anyhow!("not a noise canceling mode type one")),
            NoiseCancelingMode::Adaptive => Ok(Self::Adaptive),
            NoiseCancelingMode::Manual => Ok(Self::Manual),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[allow(clippy::enum_variant_names)]
pub enum AdaptiveNoiseCanceling {
    LowNoise,
    MediumNoise,
    HighNoise,
}

impl From<AdaptiveNoiseCanceling> for LibAdaptiveNoiseCanceling {
    fn from(value: AdaptiveNoiseCanceling) -> Self {
        match value {
            AdaptiveNoiseCanceling::LowNoise => Self::LowNoise,
            AdaptiveNoiseCanceling::MediumNoise => Self::MediumNoise,
            AdaptiveNoiseCanceling::HighNoise => Self::HighNoise,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ManualNoiseCanceling {
    Weak,
    Moderate,
    Strong,
}

impl From<ManualNoiseCanceling> for LibManualNoiseCancelingMode {
    fn from(value: ManualNoiseCanceling) -> Self {
        match value {
            ManualNoiseCanceling::Weak => Self::Weak,
            ManualNoiseCanceling::Moderate => Self::Moderate,
            ManualNoiseCanceling::Strong => Self::Strong,
        }
    }
}

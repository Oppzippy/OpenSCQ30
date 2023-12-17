#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, TransparencyMode};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub transparency_mode: TransparencyMode,
    pub custom_noise_canceling: CustomNoiseCanceling,
}

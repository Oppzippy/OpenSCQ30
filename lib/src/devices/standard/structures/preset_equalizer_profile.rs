#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr};

use super::volume_adjustments::VolumeAdjustments;

#[repr(u16)]
#[derive(
    FromRepr,
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumIter,
    EnumString,
    AsRefStr,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PresetEqualizerProfile {
    #[default]
    SoundcoreSignature = 0x0000,
    Acoustic = 0x0001,
    BassBooster = 0x0002,
    BassReducer = 0x0003,
    Classical = 0x0004,
    Podcast = 0x0005,
    Dance = 0x0006,
    Deep = 0x0007,
    Electronic = 0x0008,
    Flat = 0x0009,
    HipHop = 0x000a,
    Jazz = 0x000b,
    Latin = 0x000c,
    Lounge = 0x000d,
    Piano = 0x000e,
    Pop = 0x000f,
    RnB = 0x0010,
    Rock = 0x0011,
    SmallSpeakers = 0x0012,
    SpokenWord = 0x0013,
    TrebleBooster = 0x0014,
    TrebleReducer = 0x0015,
}

impl PresetEqualizerProfile {
    pub fn id(&self) -> u16 {
        *self as u16
    }

    pub fn from_id(id: u16) -> Option<Self> {
        Self::from_repr(id)
    }

    pub fn volume_adjustments(&self) -> VolumeAdjustments {
        let adjustments = match self {
            Self::SoundcoreSignature => [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            Self::Acoustic => [4.0, 1.0, 2.0, 2.0, 4.0, 4.0, 4.0, 2.0],
            Self::BassBooster => [4.0, 3.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            Self::BassReducer => [-4.0, -3.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            Self::Classical => [3.0, 3.0, -2.0, -2.0, 0.0, 2.0, 3.0, 4.0],
            Self::Podcast => [-3.0, 2.0, 4.0, 4.0, 3.0, 2.0, 0.0, -2.0],
            Self::Dance => [2.0, -3.0, -1.0, 1.0, 2.0, 2.0, 1.0, -3.0],
            Self::Deep => [2.0, 1.0, 3.0, 3.0, 2.0, -2.0, -4.0, -5.0],
            Self::Electronic => [3.0, 2.0, -2.0, 2.0, 1.0, 2.0, 3.0, 3.0],
            Self::Flat => [-2.0, -2.0, -1.0, 0.0, 0.0, 0.0, -2.0, -2.0],
            Self::HipHop => [2.0, 3.0, -1.0, -1.0, 2.0, -1.0, 2.0, 3.0],
            Self::Jazz => [2.0, 2.0, -2.0, -2.0, 0.0, 2.0, 3.0, 4.0],
            Self::Latin => [0.0, 0.0, -2.0, -2.0, -2.0, 0.0, 3.0, 5.0],
            Self::Lounge => [-1.0, 2.0, 4.0, 3.0, 0.0, -2.0, 2.0, 1.0],
            Self::Piano => [0.0, 3.0, 3.0, 2.0, 4.0, 5.0, 3.0, 4.0],
            Self::Pop => [-1.0, 1.0, 3.0, 3.0, 1.0, -1.0, -2.0, -3.0],
            Self::RnB => [6.0, 2.0, -2.0, -2.0, 2.0, 3.0, 3.0, 4.0],
            Self::Rock => [3.0, 2.0, -1.0, -1.0, 1.0, 3.0, 3.0, 3.0],
            Self::SmallSpeakers => [4.0, 3.0, 1.0, 0.0, -2.0, -3.0, -4.0, -4.0],
            Self::SpokenWord => [-3.0, -2.0, 1.0, 2.0, 2.0, 1.0, 0.0, -3.0],
            Self::TrebleBooster => [-2.0, -2.0, -2.0, -1.0, 1.0, 2.0, 2.0, 4.0],
            Self::TrebleReducer => [0.0, 0.0, 0.0, -2.0, -3.0, -4.0, -4.0, -6.0],
        };
        VolumeAdjustments::new(adjustments)
            .expect("all possible values are literals, so it must be a bug if this fails")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use strum::IntoEnumIterator;

    use super::PresetEqualizerProfile;

    #[test]
    fn profiles_have_unique_volume_adjustments() {
        // to make sure that nothing was mistakenly copy and pasted
        let adjustments = PresetEqualizerProfile::iter()
            .map(|profile| {
                profile
                    .volume_adjustments()
                    .adjustments()
                    .iter()
                    .cloned()
                    .map(|adjustment| (adjustment * 10.0).round() as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let deduplicated_adjustments = adjustments.iter().collect::<HashSet<_>>();
        assert_eq!(adjustments.len(), deduplicated_adjustments.len());
    }
}

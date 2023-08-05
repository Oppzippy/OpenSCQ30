use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, FromRepr};

use super::volume_adjustments::VolumeAdjustments;

#[repr(u16)]
#[derive(
    FromRepr,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Display,
    EnumIter,
    Serialize,
    Deserialize,
    EnumString,
)]
pub enum PresetEqualizerProfile {
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
        let adjustments: [i8; 8] = match self {
            Self::SoundcoreSignature => [0, 0, 0, 0, 0, 0, 0, 0],
            Self::Acoustic => [40, 10, 20, 20, 40, 40, 40, 20],
            Self::BassBooster => [40, 30, 10, 0, 0, 0, 0, 0],
            Self::BassReducer => [-40, -30, -10, 0, 0, 0, 0, 0],
            Self::Classical => [30, 30, -20, -20, 0, 20, 30, 40],
            Self::Podcast => [-30, 20, 40, 40, 30, 20, 0, -20],
            Self::Dance => [20, -30, -10, 10, 20, 20, 10, -30],
            Self::Deep => [20, 10, 30, 30, 20, -20, -40, -50],
            Self::Electronic => [30, 20, -20, 20, 10, 20, 30, 30],
            Self::Flat => [-20, -20, -10, 0, 0, 0, -20, -20],
            Self::HipHop => [20, 30, -10, -10, 20, -10, 20, 30],
            Self::Jazz => [20, 20, -20, -20, 0, 20, 30, 40],
            Self::Latin => [0, 0, -20, -20, -20, 0, 30, 50],
            Self::Lounge => [-10, 20, 40, 30, 0, -20, 20, 10],
            Self::Piano => [0, 30, 30, 20, 40, 50, 30, 40],
            Self::Pop => [-10, 10, 30, 30, 10, -10, -20, -30],
            Self::RnB => [60, 20, -20, -20, 20, 30, 30, 40],
            Self::Rock => [30, 20, -10, -10, 10, 30, 30, 30],
            Self::SmallSpeakers => [40, 30, 10, 0, -20, -30, -40, -40],
            Self::SpokenWord => [-30, -20, 10, 20, 20, 10, 0, -30],
            Self::TrebleBooster => [-20, -20, -20, -10, 10, 20, 20, 40],
            Self::TrebleReducer => [0, 0, 0, -20, -30, -40, -40, -60],
        };
        VolumeAdjustments::new(adjustments)
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
            .map(|profile| profile.volume_adjustments())
            .collect::<Vec<_>>();
        let deduplicated_adjustments = adjustments.iter().collect::<HashSet<_>>();
        assert_eq!(adjustments.len(), deduplicated_adjustments.len());
    }
}

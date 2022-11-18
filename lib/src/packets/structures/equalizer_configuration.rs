use crate::packets::structures::equalizer_band_offsets::EqualizerBandOffsets;

use super::equalizer_profile_id::EqualizerProfileId;

#[derive(Clone, Copy, Debug)]
pub enum EqualizerConfiguration {
    SoundcoreSignature,
    Acoustic,
    BassBooster,
    BassReducer,
    Classical,
    Podcast,
    Dance,
    Deep,
    Electronic,
    Flat,
    HipHop,
    Jazz,
    Latin,
    Lounge,
    Piano,
    Pop,
    RnB,
    Rock,
    SmallSpeakers,
    SpokenWord,
    TrebleBooster,
    TrebleReducer,
    Custom(EqualizerBandOffsets),
}

impl EqualizerConfiguration {
    pub fn profile_id(&self) -> EqualizerProfileId {
        match self {
            Self::SoundcoreSignature => EqualizerProfileId([0x00, 0x00]),
            Self::Acoustic => EqualizerProfileId([0x01, 0x00]),
            Self::BassBooster => EqualizerProfileId([0x02, 0x00]),
            Self::BassReducer => EqualizerProfileId([0x03, 0x00]),
            Self::Classical => EqualizerProfileId([0x04, 0x00]),
            Self::Podcast => EqualizerProfileId([0x05, 0x00]),
            Self::Dance => EqualizerProfileId([0x06, 0x00]),
            Self::Deep => EqualizerProfileId([0x07, 0x00]),
            Self::Electronic => EqualizerProfileId([0x08, 0x00]),
            Self::Flat => EqualizerProfileId([0x09, 0x00]),
            Self::HipHop => EqualizerProfileId([0x0a, 0x00]),
            Self::Jazz => EqualizerProfileId([0x0b, 0x00]),
            Self::Latin => EqualizerProfileId([0x0c, 0x00]),
            Self::Lounge => EqualizerProfileId([0x0d, 0x00]),
            Self::Piano => EqualizerProfileId([0x0e, 0x00]),
            Self::Pop => EqualizerProfileId([0x0f, 0x00]),
            Self::RnB => EqualizerProfileId([0x10, 0x00]),
            Self::Rock => EqualizerProfileId([0x11, 0x00]),
            Self::SmallSpeakers => EqualizerProfileId([0x12, 0x00]),
            Self::SpokenWord => EqualizerProfileId([0x13, 0x00]),
            Self::TrebleBooster => EqualizerProfileId([0x14, 0x00]),
            Self::TrebleReducer => EqualizerProfileId([0x15, 0x00]),
            Self::Custom(_) => EqualizerProfileId([0xfe, 0xfe]),
        }
    }

    pub fn from_profile_id(
        profile_id: EqualizerProfileId,
        band_offsets: EqualizerBandOffsets,
    ) -> Option<Self> {
        match profile_id {
            EqualizerProfileId([0x00, 0x00]) => Some(Self::SoundcoreSignature),
            EqualizerProfileId([0x01, 0x00]) => Some(Self::Acoustic),
            EqualizerProfileId([0x02, 0x00]) => Some(Self::BassBooster),
            EqualizerProfileId([0x03, 0x00]) => Some(Self::BassReducer),
            EqualizerProfileId([0x04, 0x00]) => Some(Self::Classical),
            EqualizerProfileId([0x05, 0x00]) => Some(Self::Podcast),
            EqualizerProfileId([0x06, 0x00]) => Some(Self::Dance),
            EqualizerProfileId([0x07, 0x00]) => Some(Self::Deep),
            EqualizerProfileId([0x08, 0x00]) => Some(Self::Electronic),
            EqualizerProfileId([0x09, 0x00]) => Some(Self::Flat),
            EqualizerProfileId([0x0a, 0x00]) => Some(Self::HipHop),
            EqualizerProfileId([0x0b, 0x00]) => Some(Self::Jazz),
            EqualizerProfileId([0x0c, 0x00]) => Some(Self::Latin),
            EqualizerProfileId([0x0d, 0x00]) => Some(Self::Lounge),
            EqualizerProfileId([0x0e, 0x00]) => Some(Self::Piano),
            EqualizerProfileId([0x0f, 0x00]) => Some(Self::Pop),
            EqualizerProfileId([0x10, 0x00]) => Some(Self::RnB),
            EqualizerProfileId([0x11, 0x00]) => Some(Self::Rock),
            EqualizerProfileId([0x12, 0x00]) => Some(Self::SmallSpeakers),
            EqualizerProfileId([0x13, 0x00]) => Some(Self::SpokenWord),
            EqualizerProfileId([0x14, 0x00]) => Some(Self::TrebleBooster),
            EqualizerProfileId([0x15, 0x00]) => Some(Self::TrebleReducer),
            EqualizerProfileId([0xfe, 0xfe]) => Some(Self::Custom(band_offsets)),
            _ => None,
        }
    }

    pub fn band_offsets(&self) -> EqualizerBandOffsets {
        match self {
            Self::SoundcoreSignature => EqualizerBandOffsets::new([0, 0, 0, 0, 0, 0, 0, 0]),
            Self::Acoustic => EqualizerBandOffsets::new([40, 10, 20, 20, 40, 40, 40, 20]),
            Self::BassBooster => EqualizerBandOffsets::new([40, 30, 10, 0, 0, 0, 0, 0]),
            Self::BassReducer => EqualizerBandOffsets::new([-40, -30, -10, 0, 0, 0, 0, 0]),
            Self::Classical => EqualizerBandOffsets::new([30, 30, -20, -20, 0, 20, 30, 40]),
            Self::Podcast => EqualizerBandOffsets::new([-30, 20, 40, 40, 30, 20, 0, -20]),
            Self::Dance => EqualizerBandOffsets::new([20, -30, -10, 10, 20, 20, 10, -30]),
            Self::Deep => EqualizerBandOffsets::new([20, 10, 30, 30, 20, -20, -40, -50]),
            Self::Electronic => EqualizerBandOffsets::new([30, 20, -20, 20, 10, 20, 30, 30]),
            Self::Flat => EqualizerBandOffsets::new([-20, -20, -10, 0, 0, 0, -20, -20]),
            Self::HipHop => EqualizerBandOffsets::new([20, 30, -10, -10, 20, -10, 20, 30]),
            Self::Jazz => EqualizerBandOffsets::new([20, 20, -20, -20, 0, 20, 30, 40]),
            Self::Latin => EqualizerBandOffsets::new([0, 0, -20, -20, -20, 0, 30, 50]),
            Self::Lounge => EqualizerBandOffsets::new([-10, 20, 40, 30, 0, -20, 20, 10]),
            Self::Piano => EqualizerBandOffsets::new([0, 30, 30, 20, 40, 50, 30, 40]),
            Self::Pop => EqualizerBandOffsets::new([-10, 10, 30, 30, 10, -10, -20, -30]),
            Self::RnB => EqualizerBandOffsets::new([60, 20, -20, -20, 20, 30, 30, 40]),
            Self::Rock => EqualizerBandOffsets::new([30, 20, -10, -10, 10, 30, 30, 30]),
            Self::SmallSpeakers => EqualizerBandOffsets::new([40, 30, 10, 0, -20, -30, -40, -40]),
            Self::SpokenWord => EqualizerBandOffsets::new([-30, -20, 10, 20, 20, 10, 0, -30]),
            Self::TrebleBooster => EqualizerBandOffsets::new([-20, -20, -20, -10, 10, 20, 20, 40]),
            Self::TrebleReducer => EqualizerBandOffsets::new([0, 0, 0, -20, -30, -40, -40, -60]),
            Self::Custom(values) => *values,
        }
    }
}

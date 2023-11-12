use openscq30_lib::devices::standard::structures;
use rifgen::rifgen_attr::generate_interface;

#[generate_interface]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetEqualizerProfile {
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
}

impl From<structures::PresetEqualizerProfile> for PresetEqualizerProfile {
    fn from(value: structures::PresetEqualizerProfile) -> Self {
        match value {
            structures::PresetEqualizerProfile::SoundcoreSignature => {
                PresetEqualizerProfile::SoundcoreSignature
            }
            structures::PresetEqualizerProfile::Acoustic => PresetEqualizerProfile::Acoustic,
            structures::PresetEqualizerProfile::BassBooster => PresetEqualizerProfile::BassBooster,
            structures::PresetEqualizerProfile::BassReducer => PresetEqualizerProfile::BassReducer,
            structures::PresetEqualizerProfile::Classical => PresetEqualizerProfile::Classical,
            structures::PresetEqualizerProfile::Podcast => PresetEqualizerProfile::Podcast,
            structures::PresetEqualizerProfile::Dance => PresetEqualizerProfile::Dance,
            structures::PresetEqualizerProfile::Deep => PresetEqualizerProfile::Deep,
            structures::PresetEqualizerProfile::Electronic => PresetEqualizerProfile::Electronic,
            structures::PresetEqualizerProfile::Flat => PresetEqualizerProfile::Flat,
            structures::PresetEqualizerProfile::HipHop => PresetEqualizerProfile::HipHop,
            structures::PresetEqualizerProfile::Jazz => PresetEqualizerProfile::Jazz,
            structures::PresetEqualizerProfile::Latin => PresetEqualizerProfile::Latin,
            structures::PresetEqualizerProfile::Lounge => PresetEqualizerProfile::Lounge,
            structures::PresetEqualizerProfile::Piano => PresetEqualizerProfile::Piano,
            structures::PresetEqualizerProfile::Pop => PresetEqualizerProfile::Pop,
            structures::PresetEqualizerProfile::RnB => PresetEqualizerProfile::RnB,
            structures::PresetEqualizerProfile::Rock => PresetEqualizerProfile::Rock,
            structures::PresetEqualizerProfile::SmallSpeakers => {
                PresetEqualizerProfile::SmallSpeakers
            }
            structures::PresetEqualizerProfile::SpokenWord => PresetEqualizerProfile::SpokenWord,
            structures::PresetEqualizerProfile::TrebleBooster => {
                PresetEqualizerProfile::TrebleBooster
            }
            structures::PresetEqualizerProfile::TrebleReducer => {
                PresetEqualizerProfile::TrebleReducer
            }
        }
    }
}

impl From<PresetEqualizerProfile> for structures::PresetEqualizerProfile {
    fn from(value: PresetEqualizerProfile) -> Self {
        match value {
            PresetEqualizerProfile::SoundcoreSignature => {
                structures::PresetEqualizerProfile::SoundcoreSignature
            }
            PresetEqualizerProfile::Acoustic => structures::PresetEqualizerProfile::Acoustic,
            PresetEqualizerProfile::BassBooster => structures::PresetEqualizerProfile::BassBooster,
            PresetEqualizerProfile::BassReducer => structures::PresetEqualizerProfile::BassReducer,
            PresetEqualizerProfile::Classical => structures::PresetEqualizerProfile::Classical,
            PresetEqualizerProfile::Podcast => structures::PresetEqualizerProfile::Podcast,
            PresetEqualizerProfile::Dance => structures::PresetEqualizerProfile::Dance,
            PresetEqualizerProfile::Deep => structures::PresetEqualizerProfile::Deep,
            PresetEqualizerProfile::Electronic => structures::PresetEqualizerProfile::Electronic,
            PresetEqualizerProfile::Flat => structures::PresetEqualizerProfile::Flat,
            PresetEqualizerProfile::HipHop => structures::PresetEqualizerProfile::HipHop,
            PresetEqualizerProfile::Jazz => structures::PresetEqualizerProfile::Jazz,
            PresetEqualizerProfile::Latin => structures::PresetEqualizerProfile::Latin,
            PresetEqualizerProfile::Lounge => structures::PresetEqualizerProfile::Lounge,
            PresetEqualizerProfile::Piano => structures::PresetEqualizerProfile::Piano,
            PresetEqualizerProfile::Pop => structures::PresetEqualizerProfile::Pop,
            PresetEqualizerProfile::RnB => structures::PresetEqualizerProfile::RnB,
            PresetEqualizerProfile::Rock => structures::PresetEqualizerProfile::Rock,
            PresetEqualizerProfile::SmallSpeakers => {
                structures::PresetEqualizerProfile::SmallSpeakers
            }
            PresetEqualizerProfile::SpokenWord => structures::PresetEqualizerProfile::SpokenWord,
            PresetEqualizerProfile::TrebleBooster => {
                structures::PresetEqualizerProfile::TrebleBooster
            }
            PresetEqualizerProfile::TrebleReducer => {
                structures::PresetEqualizerProfile::TrebleReducer
            }
        }
    }
}

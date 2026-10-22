//! D1203-specific feature modules (setting handlers + state modifiers).

use crate::devices::soundcore::common::modules::ModuleCollection;

mod ambient_sound_mode;
mod custom_equalizer;

pub use ambient_sound_mode::D1203AmbientSoundMode;
pub use custom_equalizer::D1203CustomEqualizer;

//! D1203-specific feature modules (setting handlers + state modifiers).

use crate::devices::soundcore::common::modules::ModuleCollection;

mod custom_equalizer;

pub use custom_equalizer::D1203CustomEqualizer;

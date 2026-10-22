//! D1203-specific feature modules (setting handlers + state modifiers).

use crate::devices::soundcore::common::modules::ModuleCollection;

mod ambient_sound_mode;
mod custom_equalizer;

pub use ambient_sound_mode::{
    D1203AmbientSoundMode, D1203ManualNoiseCanceling, D1203NoiseCancelingMode,
};
// Re-exported for the device-level integration tests in `d1203.rs`; the
// non-test code references it via its own module path, so it reads as unused
// in a plain (non-test) build.
#[allow(unused_imports)]
pub use ambient_sound_mode::build_sound_mode_payload;
pub use custom_equalizer::D1203CustomEqualizer;

//! D1203-specific feature modules (setting handlers + state modifiers).

use crate::devices::soundcore::common::modules::ModuleCollection;

mod ambient_sound_mode;
mod button_configuration;
mod custom_equalizer;
mod wind_noise;

pub use ambient_sound_mode::{
    D1203AmbientSoundMode, D1203ManualNoiseCanceling, D1203NoiseCancelingMode,
    D1203TransportationMode,
};
pub use button_configuration::D1203ButtonConfiguration;
// Re-exported for the device-level integration tests in `d1203.rs`; the
// non-test code references it via its own module path, so it reads as unused
// in a plain (non-test) build.
#[allow(unused_imports)]
pub use ambient_sound_mode::build_sound_mode_payload;
pub use custom_equalizer::D1203CustomEqualizer;
pub use wind_noise::D1203WindNoise;
// Re-exported for the device-level integration tests in `d1203.rs`; the
// non-test code references it via its own module path, so it reads as unused
// in a plain (non-test) build.
#[allow(unused_imports)]
pub use wind_noise::build_wind_noise_payload;

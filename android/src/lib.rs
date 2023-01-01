mod ambient_sound_mode;
mod equalizer_band_offsets;
mod equalizer_configuration;
mod java_glue;
mod noise_canceling_mode;
mod preset_equalizer_profile;
mod soundcore_device;
mod soundcore_device_registry;
use log::Level;
use rifgen::rifgen_attr::generate_interface;

pub use crate::ambient_sound_mode::*;
pub use crate::equalizer_band_offsets::*;
pub use crate::equalizer_configuration::*;
pub use crate::java_glue::*;
pub use crate::noise_canceling_mode::*;
pub use crate::preset_equalizer_profile::*;
pub use crate::soundcore_device::*;
pub use crate::soundcore_device_registry::*;

struct Init {}

impl Init {
    #[generate_interface]
    pub fn logging() {
        android_logger::init_once(
            android_logger::Config::default()
                .with_min_level(Level::Trace)
                .with_tag("openscq30-lib"),
        )
    }
}

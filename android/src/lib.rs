mod ambient_sound_mode;
mod btleplug_init;
mod equalizer_band_offsets;
mod equalizer_configuration;
mod java_glue;
mod noise_canceling_mode;
mod preset_equalizer_profile;
mod soundcore_device;
mod soundcore_device_registry;
mod tokio_runtime;

use rifgen::rifgen_attr::generate_interface;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
        tracing_subscriber::registry()
            .with(tracing_android::layer("openscq30-lib").unwrap())
            .try_init()
            .unwrap();
    }
}

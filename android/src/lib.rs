#![allow(clippy::inherent_to_string)]

mod devices;
mod java_glue;
mod soundcore_device_utils;
pub(crate) mod type_conversion;
use log::LevelFilter;
use rifgen::rifgen_attr::generate_interface;

pub use crate::devices::standard::packets::inbound::*;
pub use crate::devices::standard::packets::outbound::*;
pub use crate::devices::standard::structures::*;
pub use crate::java_glue::*;
pub use crate::soundcore_device_utils::*;

pub struct Init {}

impl Init {
    #[generate_interface]
    pub fn logging() {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(LevelFilter::Trace)
                .with_tag("openscq30-lib"),
        )
    }
}

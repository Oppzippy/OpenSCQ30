mod java_glue;
mod packets;
mod state;
pub(crate) mod type_conversion;
use log::Level;
use rifgen::rifgen_attr::generate_interface;

pub use crate::java_glue::*;
pub use crate::packets::inbound::*;
pub use crate::packets::outbound::*;
pub use crate::packets::structures::*;
pub use crate::state::*;

pub struct Init {}

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

mod battery_charging;
mod battery_level;
mod chinese_voice_prompt;
mod inbound_packet;
mod ldac;
mod serial_number_and_firmware_version;
mod sound_modes;
mod state;
mod tws_status;

pub use battery_charging::*;
pub use battery_level::*;
#[allow(unused_imports, reason = "used by tests, will be used in the future")]
pub use chinese_voice_prompt::*;
pub use inbound_packet::*;
#[allow(unused_imports, reason = "used by tests, will be used in the future")]
pub use ldac::*;
pub use serial_number_and_firmware_version::*;
pub use sound_modes::*;
pub use state::*;
pub use tws_status::*;

use crate::devices::soundcore::common::packet;

pub const GAMING_MODE_UPDATE_COMMAND: packet::Command = packet::Command([0x01, 0x11]);

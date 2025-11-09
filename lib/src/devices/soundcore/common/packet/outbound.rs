mod outbound_packet;
mod request_battery_charging;
mod request_battery_level;
mod request_serial_number_and_firmware_version;
mod request_state;
mod set_ambient_sound_mode_cycle;
mod set_auto_power_off;
mod set_button_configuration;
mod set_equalizer;
mod set_equalizer_and_custom_hear_id;
mod set_equalizer_with_drc;
mod set_limit_high_volume;
mod set_sound_modes;
mod set_touch_tone;

pub use outbound_packet::*;
#[allow(
    unused_imports,
    reason = "TODO consider polling with one of these every once in a while if it doesn't push this information to us"
)]
pub use request_battery_charging::*;
#[allow(
    unused_imports,
    reason = "TODO consider polling with one of these every once in a while if it doesn't push this information to us"
)]
pub use request_battery_level::*;
pub use request_serial_number_and_firmware_version::*;
pub use request_state::*;
pub use set_ambient_sound_mode_cycle::*;
pub use set_auto_power_off::*;
pub use set_button_configuration::*;
pub use set_equalizer::*;
pub use set_equalizer_and_custom_hear_id::*;
pub use set_equalizer_with_drc::*;
pub use set_limit_high_volume::*;
pub use set_sound_modes::*;
pub use set_touch_tone::*;

use crate::devices::soundcore::common::packet;

pub const SET_GAMING_MODE_COMMAND: packet::Command = packet::Command([0x10, 0x85]);

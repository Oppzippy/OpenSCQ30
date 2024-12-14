use std::collections::HashMap;

use crate::devices::standard::{packets::inbound::*, state::DeviceState};

mod battery_charging_update;
mod battery_level_update;
mod firmware_version_update;
mod sound_mode_update;
mod state_update;

pub use battery_charging_update::*;
pub use battery_level_update::*;
pub use firmware_version_update::*;
pub use sound_mode_update::*;
pub use state_update::*;
use state_update_packet::StateUpdatePacket;

use super::Command;

pub fn packet_handlers(
) -> HashMap<Command, Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
    let handlers: [(
        Command,
        Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>,
    ); 9] = [
        (
            SoundModeUpdatePacket::command(),
            Box::new(sound_mode_update_handler),
        ),
        (
            BatteryChargingUpdatePacket::command(),
            Box::new(battery_charging_update_handler),
        ),
        (
            BatteryLevelUpdatePacket::command(),
            Box::new(battery_level_update_handler),
        ),
        (
            FirmwareVersionUpdatePacket::command(),
            Box::new(firmware_version_update_handler),
        ),
        (StateUpdatePacket::command(), Box::new(state_update_handler)),
        (
            TwsStatusUpdatePacket::command(),
            Box::new(do_nothing_handler),
        ),
        (
            LdacStateUpdatePacket::command(),
            Box::new(do_nothing_handler),
        ),
        (
            ChineseVoicePromptStateUpdatePacket::command(),
            Box::new(do_nothing_handler),
        ),
        (
            GameModeUpdatePacket::command(),
            Box::new(do_nothing_handler),
        ),
    ];
    let num_handlers = handlers.len();
    let handlers_map = HashMap::from(handlers);
    debug_assert_eq!(
        num_handlers,
        handlers_map.len(),
        "there should be no duplicate packet types"
    );
    handlers_map
}

// For packets that we know how to parse, but we don't store any of the
// provided information in DeviceState
fn do_nothing_handler(_: &[u8], state: DeviceState) -> DeviceState {
    state
}

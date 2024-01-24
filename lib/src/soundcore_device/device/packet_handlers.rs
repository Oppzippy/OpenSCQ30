pub(crate) mod battery_charging_update;
pub(crate) mod battery_level_update;
pub(crate) mod firmware_version_update;
pub(crate) mod sound_mode_update;
pub(crate) mod state_update;

use std::collections::HashMap;

use crate::{
    devices::standard::{
        state::DeviceState,
        structures::{
            BATTERY_CHARGING_UPDATE, BATTERY_LEVEL_UPDATE, CHINESE_VOICE_PROMPT_STATE_UPDATE,
            FIRMWARE_VERSION_UPDATE, LDAC_STATE_UPDATE, SET_AMBIENT_SOUND_MODE_CYCLE_OK,
            SET_CUSTOM_BUTTON_MODEL_OK, SET_EQUALIZER_OK, SET_EQUALIZER_WITH_DRC_OK,
            SET_SOUND_MODE_OK, SOUND_MODE_UPDATE, STATE_UPDATE, TWS_STATUS_UPDATE,
        },
    },
    soundcore_device::device::packet_handlers::{
        battery_charging_update::battery_charging_update_handler,
        battery_level_update::battery_level_update_handler,
        firmware_version_update::firmware_version_update_handler,
        sound_mode_update::sound_mode_update_handler, state_update::state_update_handler,
    },
};

pub fn default_packet_handlers(
) -> HashMap<[u8; 7], Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>> {
    let handlers: [(
        [u8; 7],
        Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>,
    ); 13] = [
        (SOUND_MODE_UPDATE, Box::new(sound_mode_update_handler)),
        (
            BATTERY_CHARGING_UPDATE,
            Box::new(battery_charging_update_handler),
        ),
        (BATTERY_LEVEL_UPDATE, Box::new(battery_level_update_handler)),
        (
            FIRMWARE_VERSION_UPDATE,
            Box::new(firmware_version_update_handler),
        ),
        (STATE_UPDATE, Box::new(state_update_handler)),
        (SET_SOUND_MODE_OK, Box::new(do_nothing_handler)),
        (SET_EQUALIZER_OK, Box::new(do_nothing_handler)),
        (SET_EQUALIZER_WITH_DRC_OK, Box::new(do_nothing_handler)),
        (TWS_STATUS_UPDATE, Box::new(do_nothing_handler)),
        (LDAC_STATE_UPDATE, Box::new(do_nothing_handler)),
        (
            SET_AMBIENT_SOUND_MODE_CYCLE_OK,
            Box::new(do_nothing_handler),
        ),
        (SET_CUSTOM_BUTTON_MODEL_OK, Box::new(do_nothing_handler)),
        (
            CHINESE_VOICE_PROMPT_STATE_UPDATE,
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

fn do_nothing_handler(_: &[u8], state: DeviceState) -> DeviceState {
    state
}

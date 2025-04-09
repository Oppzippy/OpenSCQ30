use crate::devices::soundcore::standard::macros::soundcore_device;

use super::{packets::A3936StateUpdatePacket, state::A3936State};

soundcore_device!(A3936State, A3936StateUpdatePacket, async |builder| {
    builder.module_collection().add_state_update();
    builder.a3936_sound_modes();
    builder.equalizer_with_custom_hear_id().await;
    builder.a3936_button_configuration();
    builder.ambient_sound_mode_cycle();
    builder.dual_battery();
    builder.serial_number_and_dual_firmware_version();
});

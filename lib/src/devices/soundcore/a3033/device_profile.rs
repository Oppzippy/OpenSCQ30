use crate::devices::soundcore::standard::macros::soundcore_device;

use super::{packets::A3033StateUpdatePacket, state::A3033State};

soundcore_device!(A3033State, A3033StateUpdatePacket, async |builder| {
    builder.module_collection().add_state_update();
    builder.equalizer().await;
});

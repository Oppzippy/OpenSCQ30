use crate::devices::soundcore::standard::{
    device::fetch_state_from_state_update_packet, macros::soundcore_device,
};

use super::{packets::A3033StateUpdatePacket, state::A3033State};

soundcore_device!(
    A3033State,
    A3033StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3033State, A3033StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.equalizer().await;
        builder.single_battery();
        builder.serial_number_and_firmware_version();
    }
);

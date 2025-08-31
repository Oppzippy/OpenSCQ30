use std::collections::HashMap;

use crate::devices::soundcore::{
    a3033::{packets::A3033StateUpdatePacket, state::A3033State},
    common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        packet::outbound::{OutboundPacketBytesExt, RequestStatePacket},
    },
};

mod packets;
mod state;

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
    },
    {
        HashMap::from([(
            RequestStatePacket::COMMAND,
            A3033StateUpdatePacket::default().bytes(),
        )])
    },
);

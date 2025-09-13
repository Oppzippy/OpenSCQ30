use std::collections::HashMap;

use crate::devices::soundcore::{
    a3948::{packets::inbound::A3948StateUpdatePacket, state::A3948State},
    common::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        packet::outbound::{OutboundPacketBytesExt, RequestState},
    },
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3948State,
    A3948StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3948State, A3948StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.equalizer().await;

        builder.a3948_button_configuration();

        builder.touch_tone();

        builder.serial_number_and_dual_firmware_version();
        builder.tws_status();
        builder.dual_battery();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3948StateUpdatePacket::default().bytes(),
        )])
    },
);

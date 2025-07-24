use std::collections::HashMap;

use crate::devices::soundcore::{
    a3936::{packets::A3936StateUpdatePacket, state::A3936State},
    standard::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        packet::outbound::{OutboundPacketBytesExt, RequestStatePacket},
    },
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3936State,
    A3936StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3936State, A3936StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3936_sound_modes();
        builder.equalizer_with_custom_hear_id().await;
        builder.a3936_button_configuration();
        builder.ambient_sound_mode_cycle();
        builder.tws_status();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestStatePacket::COMMAND,
            A3936StateUpdatePacket::default().bytes(),
        )])
    },
);

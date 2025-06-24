use std::collections::HashMap;

use crate::devices::soundcore::{
    a3959::{packets::A3959StateUpdatePacket, state::A3959State},
    standard::{
        device::fetch_state_from_state_update_packet,
        macros::soundcore_device,
        packets::outbound::{OutboundPacketBytesExt, RequestStatePacket},
    },
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3959State,
    A3959StateUpdatePacket,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3959State, A3959StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3959_sound_modes();
        builder.equalizer().await;
        builder.a3959_button_configuration();
        builder.ambient_sound_mode_cycle();
        builder.dual_battery();
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestStatePacket::COMMAND,
            A3959StateUpdatePacket::default().bytes(),
        )])
    },
);

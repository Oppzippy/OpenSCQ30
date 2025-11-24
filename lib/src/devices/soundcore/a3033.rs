use std::collections::HashMap;

use crate::devices::soundcore::{
    a3033::{packets::A3033StateUpdatePacket, state::A3033State},
    common::{
        device::fetch_state_from_state_update_packet, macros::soundcore_device, modules::{equalizer}, packet::outbound::{RequestState, ToPacket}
    },
};

mod packets;
mod state;

soundcore_device!(
    A3033State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3033State, A3033StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.equalizer(equalizer::common_settings()).await;
        builder.wearing_detection();
        builder.single_battery();
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3033StateUpdatePacket::default().to_packet(),
        )])
    },
);

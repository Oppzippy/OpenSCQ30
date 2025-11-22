use std::collections::HashMap;

use crate::devices::soundcore::a3116::packets::inbound::A3116StateUpdatePacket;
use crate::devices::soundcore::a3116::state::A3116State;
use crate::devices::soundcore::common::device::SoundcoreDeviceConfig;
use crate::devices::soundcore::common::packet;
use crate::devices::soundcore::common::packet::outbound::ToPacket;
use crate::devices::soundcore::common::{
    device::fetch_state_from_state_update_packet, macros::soundcore_device,
    packet::outbound::RequestState,
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    A3116State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<_, A3116State, A3116StateUpdatePacket>(packet_io)
            .await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.a3116_equalizer().await;
        builder.a3116_volume(16);
        builder.a3116_auto_power_off();
        builder.single_battery();
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3116StateUpdatePacket::default().to_packet(),
        )])
    },
    SoundcoreDeviceConfig {
        checksum_kind: packet::ChecksumKind::None,
    },
);

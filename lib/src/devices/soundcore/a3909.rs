use std::collections::HashMap;

use uuid::uuid;

use crate::devices::soundcore::{
    a3909::{packets::A3909StateUpdatePacket, state::A3909State},
    common::{
        device::SoundcoreDeviceConfig,
        macros::soundcore_device,
        modules::equalizer,
        packet::{
            ChecksumKind,
            inbound::{SerialNumberAndFirmwareVersion, TryToPacket},
            outbound::{RequestSerialNumberAndFirmwareVersion, RequestState, ToPacket},
        },
    },
};
use crate::connection::RfcommServiceSelectionStrategy;

mod packets;
mod state;

soundcore_device!(
    A3909State,
    async |packet_io| {
        let state_update_packet: A3909StateUpdatePacket = packet_io
            .send_with_response(&RequestState::default().to_packet())
            .await?
            .try_to_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersion = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersion::default().to_packet())
            .await?
            .try_to_packet()?;
        Ok(A3909State::new(state_update_packet, sn_and_firmware))
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.equalizer_tws(equalizer::common_settings()).await;
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestState::COMMAND,
                A3909StateUpdatePacket::default().to_packet(),
            ),
            (
                RequestSerialNumberAndFirmwareVersion::COMMAND,
                SerialNumberAndFirmwareVersion::default().to_packet(),
            ),
        ])
    },
    CONFIG,
);

const CONFIG: SoundcoreDeviceConfig = SoundcoreDeviceConfig {
    checksum_kind: ChecksumKind::Suffix,
    rfcomm_service_selection_strategy: RfcommServiceSelectionStrategy::Constant(uuid!(
        "00001101-0000-1000-8000-00805f9b34fb"
    )),
};

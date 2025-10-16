use std::collections::HashMap;

use crate::devices::soundcore::{
    a3926::{packets::A3926StateUpdatePacket, state::A3926State},
    common::{
        macros::soundcore_device,
        packet::{
            inbound::{SerialNumberAndFirmwareVersion, TryIntoInboundPacket},
            outbound::{
                OutboundPacketBytesExt, RequestSerialNumberAndFirmwareVersion, RequestState,
            },
        },
    },
};

pub use crate::devices::soundcore::common::modules::button_configuration_v2::COMMON_SETTINGS as BUTTON_CONFIGURATION_SETTINGS;

mod packets;
mod state;

soundcore_device!(
    A3926State,
    A3926StateUpdatePacket,
    async |packet_io| {
        let state_update_packet: A3926StateUpdatePacket = packet_io
            .send_with_response(&RequestState::new().into())
            .await?
            .try_into_inbound_packet()?;
        let sn_and_firmware: SerialNumberAndFirmwareVersion = packet_io
            .send_with_response(&RequestSerialNumberAndFirmwareVersion::new().into())
            .await?
            .try_into_inbound_packet()?;
        Ok(A3926State::new(state_update_packet, sn_and_firmware))
    },
    async |builder| {
        builder.module_collection().add_state_update();
        // TODO confirm that this doesn't actually have sound modes and the below code is wrong
        // builder.sound_modes(AvailableSoundModes {
        //     ambient_sound_modes: vec![
        //         AmbientSoundMode::Normal,
        //         AmbientSoundMode::Transparency,
        //         AmbientSoundMode::NoiseCanceling,
        //     ],
        //     transparency_modes: Vec::new(),
        //     noise_canceling_modes: vec![
        //         NoiseCancelingMode::Transport,
        //         NoiseCancelingMode::Indoor,
        //         NoiseCancelingMode::Outdoor,
        //     ],
        // });
        builder.equalizer_with_basic_hear_id().await;
        builder.button_configuration_v2(&BUTTON_CONFIGURATION_SETTINGS);
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([
            (
                RequestState::COMMAND,
                A3926StateUpdatePacket::default().bytes(),
            ),
            (
                RequestSerialNumberAndFirmwareVersion::COMMAND,
                SerialNumberAndFirmwareVersion::default().bytes(),
            ),
        ])
    },
);

#[cfg(test)]
mod tests {
    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            device::test_utils::TestSoundcoreDevice,
            packet::{Command, Direction, Packet},
        },
        settings::{SettingId, Value},
    };

    #[tokio::test(start_paused = true)]
    async fn test_set_left_single_press() {
        let mut test_device =
            TestSoundcoreDevice::new(DeviceModel::SoundcoreA3926, super::device_registry).await;
        test_device
            .assert_set_settings_response_unordered(
                vec![(SettingId::LeftSinglePress, Value::from("PlayPause"))],
                vec![Packet {
                    direction: Direction::Outbound,
                    command: Command([0x04, 0x81]),
                    body: vec![0x00, 0x02, 0x06],
                }],
            )
            .await;
    }
}

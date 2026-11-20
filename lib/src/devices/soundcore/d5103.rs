use std::collections::HashMap;

use crate::devices::soundcore::{
    common::{
        macros::soundcore_device,
        modules::single_battery::SingleBatteryConfiguration,
        packet::{
            inbound::TryToPacket,
            outbound::{RequestState, ToPacket},
        },
    },
    d5103::{packets::inbound::D5103StateUpdatePacket, state::D5103State},
};

mod modules;
mod packets;
mod state;
mod structures;

soundcore_device!(
    D5103State,
    async |packet_io| {
        let state_update_packet: D5103StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        Ok(D5103State::new(state_update_packet))
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.d5103_equalizer().await;
        builder.d5103_bass_up();
        builder.d5103_lights();
        builder.single_battery_custom(SingleBatteryConfiguration {
            max_level: 100,
            level_offset: 0,
        });
        builder.d5103_device_information();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            D5103StateUpdatePacket::default().to_packet(),
        )])
    },
);

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, collections::HashMap};

    use nom_language::error::VerboseError;

    use crate::{
        DeviceModel,
        devices::soundcore::{
            common::{
                device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
                packet::{self, inbound::FromPacketBody, outbound::ToPacket},
            },
            d5103::packets::inbound::D5103StateUpdatePacket,
        },
        settings::{SettingId, Value},
    };

    /// GitHub issue 452, firmware 2.5.0.
    const STATE_UPDATE_BODY: [u8; 284] = [
        13, 70, 0, 1, 1, 1, 2, 50, 46, 53, 46, 48, 65, 69, 57, 68, 78, 89, 67, 48, 71, 50, 54, 50,
        48, 48, 56, 49, 54, 1, 54, 0, 0, 0, 0, 254, 184, 3, 10, 162, 6, 10, 137, 4, 10, 108, 2, 10,
        135, 3, 10, 153, 3, 10, 165, 3, 10, 144, 5, 10, 172, 2, 10, 121, 0, 0, 202, 3, 100, 0, 0,
        3, 255, 255, 255, 0, 255, 247, 0, 166, 255, 1, 11, 255, 32, 33, 255, 80, 26, 255, 156, 26,
        255, 225, 26, 164, 255, 26, 26, 255, 122, 26, 228, 255, 26, 129, 255, 125, 26, 255, 228,
        26, 255, 255, 32, 110, 2, 3, 30, 199, 255, 21, 253, 154, 131, 66, 254, 3, 4, 135, 206, 237,
        95, 233, 235, 59, 186, 240, 37, 128, 252, 4, 4, 255, 30, 0, 255, 102, 0, 255, 166, 0, 255,
        226, 0, 5, 4, 230, 39, 255, 251, 0, 105, 0, 100, 251, 255, 255, 255, 6, 4, 249, 73, 0, 255,
        128, 32, 255, 230, 175, 255, 216, 61, 7, 3, 49, 125, 0, 160, 235, 54, 255, 253, 130, 8, 1,
        255, 39, 39, 9, 1, 255, 255, 255, 10, 1, 255, 241, 155, 11, 6, 255, 27, 27, 255, 152, 42,
        57, 255, 42, 42, 162, 255, 255, 247, 42, 177, 42, 255, 12, 2, 255, 85, 0, 255, 220, 45, 13,
        5, 255, 23, 132, 255, 84, 197, 198, 0, 255, 53, 153, 255, 255, 255, 255, 14, 4, 255, 115,
        0, 249, 181, 33, 234, 255, 0, 125, 228, 1, 0, 0, 10, 0, 0, 0, 1, 9, 0, 16, 0, 17, 0, 18, 0,
        0,
    ];

    async fn new_device() -> TestSoundcoreDevice {
        TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreD5103,
            HashMap::from([(
                packet::inbound::STATE_COMMAND,
                packet::Inbound::new(packet::inbound::STATE_COMMAND, STATE_UPDATE_BODY.to_vec()),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await
    }

    #[test]
    fn state_update_packet_round_trips() {
        let (_, packet) =
            D5103StateUpdatePacket::take::<VerboseError<_>>(&STATE_UPDATE_BODY).unwrap();
        assert_eq!(packet.body(), STATE_UPDATE_BODY);
    }

    #[tokio::test(start_paused = true)]
    async fn parses_packet_from_issue_452() {
        let device = new_device().await;
        device.assert_setting_values([
            (SettingId::BatteryLevel, "70/100".into()),
            (SettingId::FirmwareVersion, "2.5.0".into()),
            (SettingId::SerialNumber, "AE9DNYC0G26200816".into()),
            (SettingId::BassUp, true.into()),
            (
                SettingId::PresetEqualizerProfile,
                Value::OptionalString(None),
            ),
            (
                SettingId::VolumeAdjustments,
                Value::I16Vec(vec![64, 42, 17, -12, 15, 33, 45, 24, 52]),
            ),
            (SettingId::LightsEnabled, false.into()),
            (SettingId::LightsBrightness, 100.into()),
            (SettingId::LightsMode, Cow::from("Combo").into()),
        ]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_bass_up_sends_flag() {
        let mut device = new_device().await;
        device
            .assert_set_settings_response(
                vec![(SettingId::BassUp, false.into())],
                vec![packet::Outbound::new(
                    packet::Command([0x02, 0x84]),
                    vec![0],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_lights_enabled_sends_toggle() {
        let mut device = new_device().await;
        device
            .assert_set_settings_response(
                vec![(SettingId::LightsEnabled, true.into())],
                vec![packet::Outbound::new(
                    packet::Command([0x0e, 0x08]),
                    vec![1],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_lights_brightness_sends_range() {
        let mut device = new_device().await;
        device
            .assert_set_settings_response(
                vec![(SettingId::LightsBrightness, 50.into())],
                vec![packet::Outbound::new(
                    packet::Command([0x0e, 0x05]),
                    vec![50],
                )],
            )
            .await;
        device.assert_setting_values([(SettingId::LightsEnabled, true.into())]);
    }

    #[tokio::test(start_paused = true)]
    async fn selecting_effect_sends_stored_body_and_turns_lights_on() {
        let mut device = new_device().await;
        device
            .assert_set_settings_response(
                vec![(SettingId::LightsMode, Cow::from("Energy").into())],
                vec![packet::Outbound::new(
                    packet::Command([0x0e, 0x07]),
                    vec![7, 3, 49, 125, 0, 160, 235, 54, 255, 253, 130],
                )],
            )
            .await;
        device.assert_setting_values([(SettingId::LightsEnabled, true.into())]);
    }

    #[tokio::test(start_paused = true)]
    async fn set_preset_sends_curve() {
        let mut device = new_device().await;
        device
            .assert_set_settings_response(
                vec![(
                    SettingId::PresetEqualizerProfile,
                    Value::OptionalString(Some("Balanced".into())),
                )],
                vec![packet::Outbound::new(
                    packet::Command([0x02, 0x88]),
                    vec![
                        2, 1, 255, 110, 3, 10, 110, 6, 10, 120, 4, 10, 110, 2, 10, 120, 3, 10, 110,
                        3, 10, 120, 3, 10, 130, 5, 10, 110, 2, 10,
                    ],
                )],
            )
            .await;
    }

    #[tokio::test(start_paused = true)]
    async fn set_custom_equalizer_sends_all_bands() {
        let mut device = new_device().await;
        device
            .assert_set_settings_response(
                vec![(
                    SettingId::VolumeAdjustments,
                    Value::I16Vec(vec![60, -60, 10, 0, 0, 0, 0, 0, -10]),
                )],
                vec![packet::Outbound::new(
                    packet::Command([0x02, 0x88]),
                    vec![
                        254, 1, 255, 180, 3, 10, 60, 6, 10, 130, 4, 10, 120, 2, 10, 120, 3, 10,
                        120, 3, 10, 120, 3, 10, 120, 5, 10, 110, 2, 10,
                    ],
                )],
            )
            .await;
    }
}

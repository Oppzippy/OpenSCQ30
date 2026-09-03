use std::collections::HashMap;

use crate::{
    devices::soundcore::{
        a3005::{packets::inbound::A3005StateUpdatePacket, state::A3005State},
        common::{
            self,
            macros::soundcore_device,
            modules::{
                equalizer::{EqualizerModuleSettings, EqualizerPreset},
                single_battery::SingleBatteryConfiguration,
            },
            packet::{
                inbound::TryToPacket,
                outbound::{RequestState, ToPacket},
            },
            structures::VolumeAdjustments,
        },
    },
    i18n::fl,
};

mod packets;
mod state;

soundcore_device!(
    A3005State,
    async |packet_io| {
        let state_update_packet: packets::inbound::A3005StateUpdatePacket = packet_io
            .send_with_response(&RequestState.to_packet())
            .await?
            .try_to_packet()?;
        let dual_connections_devices = if state_update_packet.dual_connections_enabled {
            common::modules::dual_connections::take_dual_connection_devices(&packet_io).await?
        } else {
            Vec::new()
        };
        Ok(state::A3005State::new(
            state_update_packet,
            dual_connections_devices,
        ))
    },
    async |builder| {
        builder.module_collection().add_state_update();

        builder.equalizer_with_drc(equalizer_settings()).await;

        builder.dual_connections();

        builder.auto_power_off(
            common::modules::auto_power_off::AutoPowerOffDuration::half_hour_increments(),
        );

        builder.single_battery_custom(SingleBatteryConfiguration {
            max_level: 10,
            level_offset: 1,
        });
        builder.serial_number_and_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3005StateUpdatePacket::default().to_packet(),
        )])
    },
);

fn equalizer_settings() -> EqualizerModuleSettings<8, 10, -120, 134, 1> {
    common::modules::equalizer::common_settings_with_presets(vec![
        EqualizerPreset {
            name: "SoundcoreSignature",
            localized_name: || fl!("soundcore-signature"),
            id: 0,
            volume_adjustments: VolumeAdjustments::new([0, 0, 0, 0, 0, 0, 0, 0, 0, -120]),
        },
        EqualizerPreset {
            name: "Acoustic",
            localized_name: || fl!("acoustic"),
            id: 1,
            volume_adjustments: VolumeAdjustments::new([40, 10, 20, 20, 40, 40, 40, 20, 0, -120]),
        },
        EqualizerPreset {
            name: "BassReducer",
            localized_name: || fl!("bass-reducer"),
            id: 3,
            volume_adjustments: VolumeAdjustments::new([-40, -30, -10, 0, 0, 0, 0, 0, 0, -120]),
        },
        EqualizerPreset {
            name: "Classical",
            localized_name: || fl!("classical"),
            id: 4,
            volume_adjustments: VolumeAdjustments::new([30, 30, -20, -20, 0, 20, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Podcast",
            localized_name: || fl!("podcast"),
            id: 5,
            volume_adjustments: VolumeAdjustments::new([-30, 20, 40, 40, 30, 20, 0, -20, 0, -120]),
        },
        EqualizerPreset {
            name: "Dance",
            localized_name: || fl!("dance"),
            id: 6,
            volume_adjustments: VolumeAdjustments::new([
                20, -30, -10, 10, 20, 20, 10, -30, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "Deep",
            localized_name: || fl!("deep"),
            id: 7,
            volume_adjustments: VolumeAdjustments::new([
                20, 10, 30, 30, 20, -20, -40, -50, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "Electronic",
            localized_name: || fl!("electronic"),
            id: 8,
            volume_adjustments: VolumeAdjustments::new([30, 20, -20, 20, 10, 20, 30, 30, 0, -120]),
        },
        EqualizerPreset {
            name: "Flat",
            localized_name: || fl!("flat"),
            id: 9,
            volume_adjustments: VolumeAdjustments::new([-20, -20, -10, 0, 0, 0, -20, -20, 0, -120]),
        },
        EqualizerPreset {
            name: "HipHop",
            localized_name: || fl!("hip-hop"),
            id: 10,
            volume_adjustments: VolumeAdjustments::new([
                20, 30, -10, -10, 20, -10, 20, 30, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "Jazz",
            localized_name: || fl!("jazz"),
            id: 11,
            volume_adjustments: VolumeAdjustments::new([20, 20, -20, -20, 0, 20, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Latin",
            localized_name: || fl!("latin"),
            id: 12,
            volume_adjustments: VolumeAdjustments::new([0, 0, -20, -20, -20, 0, 30, 50, 0, -120]),
        },
        EqualizerPreset {
            name: "Lounge",
            localized_name: || fl!("lounge"),
            id: 13,
            volume_adjustments: VolumeAdjustments::new([-10, 20, 40, 30, 0, -20, 20, 10, 0, -120]),
        },
        EqualizerPreset {
            name: "Piano",
            localized_name: || fl!("piano"),
            id: 14,
            volume_adjustments: VolumeAdjustments::new([0, 30, 30, 20, 40, 50, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Pop",
            localized_name: || fl!("pop"),
            id: 15,
            volume_adjustments: VolumeAdjustments::new([
                -10, 10, 30, 30, 10, -10, -20, -30, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "RnB",
            localized_name: || fl!("rnb"),
            id: 16,
            volume_adjustments: VolumeAdjustments::new([60, 20, -20, -20, 20, 30, 30, 40, 0, -120]),
        },
        EqualizerPreset {
            name: "Rock",
            localized_name: || fl!("rock"),
            id: 17,
            volume_adjustments: VolumeAdjustments::new([30, 20, -10, -10, 10, 30, 40, 50, 0, -120]),
        },
        EqualizerPreset {
            name: "SmallSpeakers",
            localized_name: || fl!("small-speakers"),
            id: 18,
            volume_adjustments: VolumeAdjustments::new([
                40, 30, 10, 0, -20, -30, -40, -40, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "SpokenWord",
            localized_name: || fl!("spoken-word"),
            id: 19,
            volume_adjustments: VolumeAdjustments::new([-30, -20, 10, 20, 20, 10, 0, -30, 0, -120]),
        },
        EqualizerPreset {
            name: "TrebleBooster",
            localized_name: || fl!("treble-booster"),
            id: 20,
            volume_adjustments: VolumeAdjustments::new([
                -20, -20, -20, -10, 10, 20, 20, 40, 0, -120,
            ]),
        },
        EqualizerPreset {
            name: "TrebleReducer",
            localized_name: || fl!("treble-reducer"),
            id: 21,
            volume_adjustments: VolumeAdjustments::new([0, 0, 0, -20, -30, -40, -40, -60, 0, -120]),
        },
        EqualizerPreset {
            name: "BassBooster",
            localized_name: || fl!("bass-booster"),
            id: 32382,
            volume_adjustments: VolumeAdjustments::new([40, 30, 10, 0, 0, 0, 0, 0, 0, -120]),
        },
    ])
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, collections::HashMap};

    use crate::{
        DeviceModel,
        devices::soundcore::common::{
            device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
            packet,
        },
        settings::SettingId,
    };

    #[tokio::test(start_paused = true)]
    async fn matches_soundcore_app() {
        let device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3005,
            HashMap::from([(
                packet::Command([1, 1]),
                packet::Inbound::new(
                    packet::Command([1, 1]),
                    vec![
                        7, 0, 48, 49, 46, 50, 49, 51, 48, 48, 53, 66, 65, 66, 65, 70, 66, 50, 67,
                        57, 67, 49, 56, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 0, 255,
                        255, 255, 255, 255, 255, 1, 49, 0, 0, 0, 1, 1, 255, 255, 255, 255, 255,
                        255, 255,
                    ],
                ),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;
        device.assert_setting_values([
            (SettingId::BatteryLevel, "8/10".into()),
            (SettingId::IsCharging, "No".into()),
            (SettingId::FirmwareVersion, "01.21".into()),
            (SettingId::SerialNumber, "3005BABAFB2C9C18".into()),
            (
                SettingId::PresetEqualizerProfile,
                Some("SoundcoreSignature").into(),
            ),
            (SettingId::AutoPowerOff, Cow::from("60m").into()),
        ]);
    }
}

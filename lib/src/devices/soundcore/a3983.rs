use std::collections::HashMap;

use openscq30_lib_has::Has;

use crate::devices::soundcore::{
    a3933::{
        BUTTON_CONFIGURATION_SETTINGS, packets::inbound::A3933StateUpdatePacket, state::A3933State,
    },
    common::{
        device::fetch_state_from_state_update_packet,
        macros::{soundcore_device, state_update_packet_module},
        modules::{equalizer, sound_modes::AvailableSoundModes},
        packet::{
            self,
            outbound::{RequestState, ToPacket},
        },
        structures::AmbientSoundMode,
    },
};

// The A3983 uses the A3933 state layout, including its unused HearID fields.
// Only expose features available on the Life Note 3i.
soundcore_device!(
    A3983State,
    async |packet_io| {
        fetch_state_from_state_update_packet::<A3983State, A3933StateUpdatePacket>(packet_io).await
    },
    async |builder| {
        builder.module_collection().add_state_update();
        builder.sound_modes(AvailableSoundModes {
            ambient_sound_modes: vec![
                AmbientSoundMode::Normal,
                AmbientSoundMode::Transparency,
                AmbientSoundMode::NoiseCanceling,
            ],
            transparency_modes: vec![],
            noise_canceling_modes: vec![],
        });
        builder.equalizer_tws(equalizer::common_settings()).await;
        builder.button_configuration(&BUTTON_CONFIGURATION_SETTINGS);
        builder.ambient_sound_mode_cycle();
        builder.reset_button_configuration::<A3933StateUpdatePacket>(RequestState.to_packet());
        builder.tws_status();
        builder.dual_battery(5);
        builder.serial_number_and_dual_firmware_version();
    },
    {
        HashMap::from([(
            RequestState::COMMAND,
            A3933StateUpdatePacket::default().to_packet(),
        )])
    },
);

// A distinct type lets this model build a different feature set from the A3933.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3983State(A3933State);

impl<T> Has<T> for A3983State
where
    A3933State: Has<T>,
{
    fn get(&self) -> &T {
        self.0.get()
    }

    fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
}

impl From<A3933StateUpdatePacket> for A3983State {
    fn from(value: A3933StateUpdatePacket) -> Self {
        Self(value.into())
    }
}

state_update_packet_module!(A3983State, A3933StateUpdatePacket);

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::{
        api::settings::{SettingId, Value},
        devices::{
            DeviceModel,
            soundcore::common::{
                device::{SoundcoreDeviceConfig, test_utils::TestSoundcoreDevice},
                packet::{self, inbound::FromPacketBody},
                structures::HostDevice,
            },
        },
    };

    use super::*;

    // Captured from an A3983 running firmware 11.06; serial number anonymized.
    const CAPTURED_BODY: &[u8] = &[
        1, 0, 5, 5, 0, 0, 49, 49, 46, 48, 54, 49, 49, 46, 48, 54, 51, 57, 56, 51, 48, 48, 48, 48,
        48, 48, 48, 48, 48, 48, 48, 48, 0, 0, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 99, 1, 84, 1, 102, 1, 84, 1, 1, 1, 0, 5, 2, 0, 1, 0, 255, 255, 0, 0, 0,
        0, 0, 49, 0, 0, 0, 0,
    ];

    #[test]
    fn parses_life_note_3i_capture() {
        let (remainder, state) =
            A3933StateUpdatePacket::take::<VerboseError<_>>(CAPTURED_BODY).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(state.tws_status.host_device, HostDevice::Right);
        assert!(!state.tws_status.is_connected);
        assert_eq!(state.battery.left.level.0, 5);
        assert_eq!(state.battery.right.level.0, 5);
        assert_eq!(state.serial_number.as_str(), "3983000000000000");
        assert_eq!(
            state.sound_modes.ambient_sound_mode,
            AmbientSoundMode::Normal
        );
    }

    #[tokio::test(start_paused = true)]
    async fn connects_using_captured_state() {
        let mut device = TestSoundcoreDevice::new(
            super::device_registry,
            DeviceModel::SoundcoreA3983,
            HashMap::from([(
                RequestState::COMMAND,
                packet::Inbound::new(RequestState::COMMAND, CAPTURED_BODY.to_vec()),
            )]),
            SoundcoreDeviceConfig::default(),
        )
        .await;

        device.assert_setting_values(vec![
            (SettingId::AmbientSoundMode, Value::String("Normal".into())),
            (
                SettingId::FirmwareVersionLeft,
                Value::String("11.06".into()),
            ),
            (
                SettingId::FirmwareVersionRight,
                Value::String("11.06".into()),
            ),
            (
                SettingId::SerialNumber,
                Value::String("3983000000000000".into()),
            ),
        ]);
        for unsupported in [
            SettingId::NoiseCancelingMode,
            SettingId::TransparencyMode,
            SettingId::GamingMode,
            SettingId::WearingDetection,
            SettingId::CaseBatteryLevel,
        ] {
            assert!(!device.inner().categories().into_iter().any(|category| {
                device
                    .inner()
                    .settings_in_category(&category)
                    .contains(&unsupported)
            }));
        }
        let (_, mut state) =
            A3933StateUpdatePacket::take::<VerboseError<_>>(CAPTURED_BODY).unwrap();
        state.sound_modes.ambient_sound_mode = AmbientSoundMode::NoiseCanceling;
        device
            .assert_set_settings_response(
                vec![(
                    SettingId::AmbientSoundMode,
                    Value::String("NoiseCanceling".into()),
                )],
                vec![packet::outbound::SetSoundModes(state.sound_modes).to_packet()],
            )
            .await;
    }
}

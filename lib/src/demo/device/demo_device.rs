use std::time::Duration;

use async_trait::async_trait;
use macaddr::MacAddr6;
use tokio::sync::{broadcast, watch, Mutex};

use crate::{
    api::{connection::ConnectionStatus, device::Device},
    packets::structures::{
        AgeRange, AmbientSoundMode, BasicHearId, BatteryLevel, DeviceFeatureFlags,
        EqualizerConfiguration, FirmwareVersion, IsBatteryCharging, NoiseCancelingMode,
        PresetEqualizerProfile, SerialNumber, SingleBattery, SoundModes,
    },
    state::DeviceState,
};

pub struct DemoDevice {
    name: String,
    mac_address: MacAddr6,
    state: Mutex<DeviceState>,
    state_sender: broadcast::Sender<DeviceState>,
    connection_status_sender: watch::Sender<ConnectionStatus>,
}

impl DemoDevice {
    pub async fn new(name: impl Into<String>, mac_address: MacAddr6) -> Self {
        tokio::time::sleep(Duration::from_millis(500)).await; // it takes some time to connect
        let (state_sender, _) = broadcast::channel(50);
        let (connection_status_sender, _) = watch::channel(ConnectionStatus::Connected);
        Self {
            name: name.into(),
            mac_address,
            state_sender,
            connection_status_sender,
            state: Mutex::new(DeviceState {
                feaure_flags: DeviceFeatureFlags::all(),
                battery: SingleBattery {
                    is_charging: IsBatteryCharging::No,
                    level: BatteryLevel(4),
                }
                .into(),
                equalizer_configuration: EqualizerConfiguration::new_from_preset_profile(
                    PresetEqualizerProfile::SoundcoreSignature,
                ),
                sound_modes: Some(SoundModes {
                    ambient_sound_mode: AmbientSoundMode::Normal,
                    noise_canceling_mode: NoiseCancelingMode::Indoor,
                    transparency_mode: Default::default(),
                    custom_noise_canceling: Default::default(),
                }),
                age_range: Some(AgeRange(0)),
                custom_button_model: None,
                custom_hear_id: Some(
                    BasicHearId {
                        is_enabled: true,
                        time: 0,
                        volume_adjustments: Default::default(),
                    }
                    .into(),
                ),
                firmware_version: Some(FirmwareVersion("01.00".into())),
                serial_number: Some(SerialNumber("0123456789ABCDEF".into())),
            }),
        }
    }
}

#[async_trait]
impl Device for DemoDevice {
    fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState> {
        self.state_sender.subscribe()
    }

    async fn mac_address(&self) -> crate::Result<MacAddr6> {
        Ok(self.mac_address.to_owned())
    }

    async fn name(&self) -> crate::Result<String> {
        Ok(self.name.to_owned())
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_sender.subscribe()
    }

    async fn state(&self) -> DeviceState {
        self.state.lock().await.clone()
    }

    async fn set_sound_modes(&self, sound_modes: SoundModes) -> crate::Result<()> {
        tracing::info!("set sound modes to {sound_modes:?}");
        let mut state = self.state.lock().await;
        *state = DeviceState {
            sound_modes: Some(sound_modes),
            ..state.clone()
        };
        Ok(())
    }

    async fn set_equalizer_configuration(
        &self,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<()> {
        tracing::info!("set equalizer configuration to {equalizer_configuration:?}");
        let mut state = self.state.lock().await;
        *state = DeviceState {
            equalizer_configuration,
            ..state.clone()
        };
        Ok(())
    }
}

impl core::fmt::Debug for DemoDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DemoDevice")
            .field("name", &self.name)
            .field("mac_address", &self.mac_address)
            .field("state", &self.state)
            .field("state_sender", &self.state_sender)
            .finish()
    }
}

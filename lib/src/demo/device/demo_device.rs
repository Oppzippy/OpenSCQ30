use std::{marker::PhantomData, sync::Arc, time::Duration};

use macaddr::MacAddr6;
use tokio::sync::{watch, Mutex};
use uuid::Uuid;

use crate::{
    api::{connection::ConnectionStatus, device::Device},
    device_profiles::{
        DeviceProfile, NoiseCancelingModeType, SoundModeProfile, TransparencyModeType,
    },
    devices::standard::{
        state::DeviceState,
        structures::{
            AgeRange, AmbientSoundMode, AmbientSoundModeCycle, BasicHearId, BatteryLevel,
            ButtonAction, CustomButtonModel, EqualizerConfiguration, FirmwareVersion, Gender,
            HearId, IsBatteryCharging, NoTwsButtonAction, NoiseCancelingMode,
            PresetEqualizerProfile, SerialNumber, SingleBattery, SoundModes, TwsButtonAction,
        },
    },
    futures::Futures,
};

pub struct DemoDevice<FuturesType> {
    name: String,
    mac_address: MacAddr6,
    state_sender: Arc<Mutex<watch::Sender<DeviceState>>>,
    connection_status_sender: watch::Sender<ConnectionStatus>,
    futures: PhantomData<FuturesType>,
}

impl<FuturesType> DemoDevice<FuturesType>
where
    FuturesType: Futures,
{
    pub async fn new(name: impl Into<String>, mac_address: MacAddr6) -> Self {
        FuturesType::sleep(Duration::from_millis(500)).await; // it takes some time to connect
                                                              //
        let (state_sender, _) = watch::channel(DeviceState {
            device_profile: DeviceProfile {
                sound_mode: Some(SoundModeProfile {
                    noise_canceling_mode_type: NoiseCancelingModeType::Custom,
                    transparency_mode_type: TransparencyModeType::Custom,
                }),
                has_hear_id: true,
                num_equalizer_channels: 2,
                num_equalizer_bands: 10,
                has_dynamic_range_compression: true,
                dynamic_range_compression_min_firmware_version: None,
                has_custom_button_model: true,
                has_wear_detection: true,
                has_touch_tone: true,
                has_auto_power_off: true,
                has_ambient_sound_mode_cycle: true,
                custom_dispatchers: None,
            },
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
            gender: Some(Gender(0)),
            age_range: Some(AgeRange(0)),
            custom_button_model: Some(CustomButtonModel {
                left_double_click: TwsButtonAction {
                    tws_connected_action: ButtonAction::NextSong,
                    tws_disconnected_action: ButtonAction::PlayPause,
                    is_enabled: true,
                },
                left_long_press: TwsButtonAction {
                    tws_connected_action: ButtonAction::PreviousSong,
                    tws_disconnected_action: ButtonAction::Trans,
                    is_enabled: true,
                },
                right_double_click: TwsButtonAction {
                    tws_connected_action: ButtonAction::VoiceAssistant,
                    tws_disconnected_action: ButtonAction::VolumeDown,
                    is_enabled: true,
                },
                right_long_press: TwsButtonAction {
                    tws_connected_action: ButtonAction::VolumeUp,
                    tws_disconnected_action: ButtonAction::NextSong,
                    is_enabled: false,
                },
                left_single_click: NoTwsButtonAction {
                    action: ButtonAction::PreviousSong,
                    is_enabled: true,
                },
                right_single_click: NoTwsButtonAction {
                    action: ButtonAction::NextSong,
                    is_enabled: false,
                },
            }),
            hear_id: Some(
                BasicHearId {
                    is_enabled: true,
                    time: 0,
                    volume_adjustments: Default::default(),
                }
                .into(),
            ),
            firmware_version: Some(FirmwareVersion::new(2, 0)),
            serial_number: Some(SerialNumber("0123456789ABCDEF".into())),
            ambient_sound_mode_cycle: Some(AmbientSoundModeCycle::default()),
        });

        let (connection_status_sender, _) = watch::channel(ConnectionStatus::Connected);

        Self {
            name: name.into(),
            mac_address,
            state_sender: Arc::new(Mutex::new(state_sender)),
            connection_status_sender,
            futures: PhantomData,
        }
    }
}

impl<FuturesType> Device for DemoDevice<FuturesType>
where
    FuturesType: Futures,
{
    async fn subscribe_to_state_updates(&self) -> watch::Receiver<DeviceState> {
        self.state_sender.lock().await.subscribe()
    }

    async fn mac_address(&self) -> crate::Result<MacAddr6> {
        Ok(self.mac_address.to_owned())
    }

    async fn name(&self) -> crate::Result<String> {
        Ok(self.name.to_owned())
    }

    fn service_uuid(&self) -> Uuid {
        Uuid::default()
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status_sender.subscribe()
    }

    async fn state(&self) -> DeviceState {
        self.state_sender.lock().await.borrow().to_owned()
    }

    async fn set_sound_modes(&self, sound_modes: SoundModes) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();
        if state.sound_modes.is_none() {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "sound modes",
            });
        }
        if state.sound_modes == Some(sound_modes) {
            return Ok(());
        }
        tracing::info!("set sound modes to {sound_modes:?}");
        state_sender.send_replace(DeviceState {
            sound_modes: Some(sound_modes),
            ..state
        });
        Ok(())
    }

    async fn set_ambient_sound_mode_cycle(
        &self,
        cycle: AmbientSoundModeCycle,
    ) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();
        if state.sound_modes.is_none() {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "ambient sound mode cycle",
            });
        }
        if state.ambient_sound_mode_cycle == Some(cycle) {
            return Ok(());
        }
        tracing::info!("set ambient sound mode cycle to {cycle:?}");
        state_sender.send_replace(DeviceState {
            ambient_sound_mode_cycle: Some(cycle),
            ..state
        });
        Ok(())
    }

    async fn set_equalizer_configuration(
        &self,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();
        if state.equalizer_configuration == equalizer_configuration {
            return Ok(());
        }
        tracing::info!("set equalizer configuration to {equalizer_configuration:?}");
        state_sender.send_replace(DeviceState {
            equalizer_configuration,
            ..state
        });
        Ok(())
    }

    async fn set_hear_id(&self, hear_id: HearId) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();
        if state.hear_id.is_none() {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "hear id",
            });
        }
        if state.hear_id.as_ref() == Some(&hear_id) {
            return Ok(());
        }
        tracing::info!("set hear id to {hear_id:?}");
        state_sender.send_replace(DeviceState {
            hear_id: Some(hear_id),
            ..state
        });
        Ok(())
    }

    async fn set_custom_button_model(
        &self,
        custom_button_model: CustomButtonModel,
    ) -> crate::Result<()> {
        let state_sender = self.state_sender.lock().await;
        let state = state_sender.borrow().to_owned();
        if state.custom_button_model.is_none() {
            return Err(crate::Error::FeatureNotSupported {
                feature_name: "custom button model",
            });
        }
        if state.custom_button_model == Some(custom_button_model) {
            return Ok(());
        }
        tracing::info!("set custom button model to {custom_button_model:?}");
        state_sender.send_replace(DeviceState {
            custom_button_model: Some(custom_button_model),
            ..state
        });
        Ok(())
    }
}

impl<FuturesType> core::fmt::Debug for DemoDevice<FuturesType>
where
    FuturesType: Futures,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DemoDevice")
            .field("name", &self.name)
            .field("mac_address", &self.mac_address)
            .field("state_sender", &self.state_sender)
            .finish()
    }
}

use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use macaddr::MacAddr6;
use openscq30_lib::{
    api::{connection::ConnectionStatus, device::Device},
    packets::structures::{AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode},
    state::DeviceState,
};
use tokio::{
    runtime::Runtime,
    sync::{broadcast, watch},
};

pub struct GtkDevice<InnerDeviceType: 'static>
where
    InnerDeviceType: Device + Send + Sync,
{
    tokio_runtime: Arc<Runtime>,
    soundcore_device: Arc<InnerDeviceType>,
}

impl<InnerDeviceType> GtkDevice<InnerDeviceType>
where
    InnerDeviceType: Device + Send + Sync,
{
    pub fn new(device: Arc<InnerDeviceType>, tokio_runtime: Arc<Runtime>) -> Self {
        Self {
            tokio_runtime,
            soundcore_device: device,
        }
    }
}

#[async_trait]
impl<InnerDeviceType> Device for GtkDevice<InnerDeviceType>
where
    InnerDeviceType: Device + Send + Sync,
{
    fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState> {
        self.soundcore_device.subscribe_to_state_updates()
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.soundcore_device.connection_status()
    }

    async fn mac_address(&self) -> openscq30_lib::Result<MacAddr6> {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn(async move { soundcore_device.mac_address().await })
            .await
            .unwrap()
    }

    async fn name(&self) -> openscq30_lib::Result<String> {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn(async move { soundcore_device.name().await })
            .await
            .unwrap()
    }

    async fn ambient_sound_mode(&self) -> AmbientSoundMode {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn(async move { soundcore_device.ambient_sound_mode().await })
            .await
            .unwrap()
    }

    async fn set_ambient_sound_mode(
        &self,
        ambient_sound_mode: AmbientSoundMode,
    ) -> openscq30_lib::Result<()> {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn(async move {
                soundcore_device
                    .set_ambient_sound_mode(ambient_sound_mode)
                    .await
            })
            .await
            .unwrap()
    }

    async fn noise_canceling_mode(&self) -> NoiseCancelingMode {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn(async move { soundcore_device.noise_canceling_mode().await })
            .await
            .unwrap()
    }

    async fn set_noise_canceling_mode(
        &self,
        noise_canceling_mode: NoiseCancelingMode,
    ) -> openscq30_lib::Result<()> {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn(async move {
                soundcore_device
                    .set_noise_canceling_mode(noise_canceling_mode)
                    .await
            })
            .await
            .unwrap()
    }

    async fn equalizer_configuration(&self) -> EqualizerConfiguration {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn(async move { soundcore_device.equalizer_configuration().await })
            .await
            .unwrap()
    }

    async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> openscq30_lib::Result<()> {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn(async move {
                soundcore_device
                    .set_equalizer_configuration(configuration)
                    .await
            })
            .await
            .unwrap()
    }
}

impl<InnerDeviceType> Debug for GtkDevice<InnerDeviceType>
where
    InnerDeviceType: Device + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("GtkDevice")
    }
}

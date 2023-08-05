use std::{fmt::Debug, rc::Rc};

use async_trait::async_trait;
use macaddr::MacAddr6;
use openscq30_lib::{
    api::{connection::ConnectionStatus, device::Device},
    packets::structures::{EqualizerConfiguration, SoundModes},
    state::DeviceState,
};
use tokio::{
    runtime::Runtime,
    sync::{broadcast, watch},
};

use super::tokio_spawn_local::TokioSpawnLocal;

pub struct GtkDevice<InnerDeviceType: 'static>
where
    InnerDeviceType: Device,
{
    tokio_runtime: Rc<Runtime>,
    soundcore_device: Rc<InnerDeviceType>,
}

impl<InnerDeviceType> GtkDevice<InnerDeviceType>
where
    InnerDeviceType: Device,
{
    pub fn new(device: Rc<InnerDeviceType>, tokio_runtime: Rc<Runtime>) -> Self {
        Self {
            tokio_runtime,
            soundcore_device: device,
        }
    }
}

#[async_trait(?Send)]
impl<InnerDeviceType> Device for GtkDevice<InnerDeviceType>
where
    InnerDeviceType: Device,
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
            .spawn_local(async move { soundcore_device.mac_address().await })
            .await
            .unwrap()
    }

    async fn name(&self) -> openscq30_lib::Result<String> {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn_local(async move { soundcore_device.name().await })
            .await
            .unwrap()
    }

    async fn state(&self) -> DeviceState {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn_local(async move { soundcore_device.state().await })
            .await
            .unwrap()
    }

    async fn set_sound_modes(&self, sound_modes: SoundModes) -> openscq30_lib::Result<()> {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn_local(async move { soundcore_device.set_sound_modes(sound_modes).await })
            .await
            .unwrap()
    }

    async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> openscq30_lib::Result<()> {
        let soundcore_device = self.soundcore_device.to_owned();
        self.tokio_runtime
            .spawn_local(async move {
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
    InnerDeviceType: Device,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("GtkDevice")
    }
}

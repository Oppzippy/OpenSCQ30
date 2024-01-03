use std::sync::Arc;

use macaddr::MacAddr6;
use openscq30_lib::{
    api::device::Device,
    demo::device::DemoDevice,
    devices::standard::{
        state::DeviceState,
        structures::{
            AmbientSoundModeCycle, CustomButtonModel, EqualizerConfiguration, HearId, SoundModes,
        },
    },
    futures::TokioFutures,
    soundcore_device::device::SoundcoreDevice,
};
use thiserror::Error;
use tokio::{runtime::Runtime, sync::watch};
use uuid::Uuid;

use crate::connection::ManualConnection;

#[derive(Error, Debug, uniffi::Error)]
#[uniffi(flat_error)]
pub enum DeviceError {
    #[error(transparent)]
    OpenSCQ30Native(#[from] openscq30_lib::Error),
    #[error(transparent)]
    Protobuf(#[from] prost::DecodeError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[uniffi::export(callback_interface)]
pub trait NativeDeviceStateObserver: Send + Sync {
    fn on_state_changed(&self, device_state: DeviceState);
}

#[uniffi::export(callback_interface)]
pub trait NativeConnectionStatusObserver: Send + Sync {
    fn on_status_changed(&self, connection_status: Vec<u8>);
}

#[derive(uniffi::Object)]
pub struct NativeSoundcoreDevice {
    device: Arc<DeviceImplementation>,
    runtime: Runtime,
}

#[uniffi::export]
pub async fn new_soundcore_device(
    connection: Arc<ManualConnection>,
) -> Result<Arc<NativeSoundcoreDevice>, DeviceError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .build()?;
    let device = Arc::new(DeviceImplementation::Manual({
        let connection = connection.to_owned();
        runtime
            .spawn(async move { SoundcoreDevice::new(connection).await })
            .await
            .unwrap()?
    }));
    Ok(Arc::new(NativeSoundcoreDevice { device, runtime }))
}

#[uniffi::export]
pub async fn new_demo_soundcore_device(
    name: String,
    mac_address: MacAddr6,
) -> Result<Arc<NativeSoundcoreDevice>, DeviceError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .build()?;
    let device = Arc::new(DeviceImplementation::Demo(
        runtime
            .spawn(async move { DemoDevice::<TokioFutures>::new(name, mac_address).await })
            .await
            .unwrap(),
    ));
    Ok(Arc::new(NativeSoundcoreDevice { device, runtime }))
}

#[uniffi::export]
impl NativeSoundcoreDevice {
    pub fn subscribe_to_state_updates(&self, observer: Box<dyn NativeDeviceStateObserver>) {
        let device = self.device.to_owned();
        self.runtime.spawn(async move {
            let mut receiver = device.subscribe_to_state_updates().await;
            while receiver.changed().await.is_ok() {
                observer.on_state_changed(receiver.borrow().to_owned());
            }
        });
    }

    pub async fn mac_address(&self) -> Result<String, DeviceError> {
        self.device
            .mac_address()
            .await
            .map(|mac_address| mac_address.to_string())
            .map_err(Into::into)
    }

    pub fn service_uuid(&self) -> Uuid {
        self.device.service_uuid()
    }

    pub async fn name(&self) -> Result<String, DeviceError> {
        self.device.name().await.map_err(Into::into)
    }

    pub fn connection_status(&self, _observer: Box<dyn NativeConnectionStatusObserver>) {}

    pub async fn state(&self) -> DeviceState {
        self.device.state().await
    }

    pub async fn set_sound_modes(&self, sound_modes: SoundModes) -> Result<(), DeviceError> {
        self.device
            .set_sound_modes(sound_modes)
            .await
            .map_err(Into::into)
    }

    pub async fn set_ambient_sound_mode_cycle(
        &self,
        cycle: AmbientSoundModeCycle,
    ) -> Result<(), DeviceError> {
        self.device
            .set_ambient_sound_mode_cycle(cycle)
            .await
            .map_err(Into::into)
    }

    pub async fn set_equalizer_configuration(
        &self,
        equalizer_configuration: EqualizerConfiguration,
    ) -> Result<(), DeviceError> {
        self.device
            .set_equalizer_configuration(equalizer_configuration)
            .await
            .map_err(Into::into)
    }

    pub async fn set_hear_id(&self, hear_id: HearId) -> Result<(), DeviceError> {
        self.device.set_hear_id(hear_id).await.map_err(Into::into)
    }

    pub async fn set_custom_button_model(
        &self,
        custom_button_model: CustomButtonModel,
    ) -> Result<(), DeviceError> {
        self.device
            .set_custom_button_model(custom_button_model)
            .await
            .map_err(Into::into)
    }
}

// Dynamic dispatch does not work with async functions in traits
enum DeviceImplementation {
    Manual(SoundcoreDevice<ManualConnection, TokioFutures>),
    Demo(DemoDevice<TokioFutures>),
}

impl DeviceImplementation {
    pub async fn subscribe_to_state_updates(&self) -> watch::Receiver<DeviceState> {
        match self {
            DeviceImplementation::Manual(device) => device.subscribe_to_state_updates().await,
            DeviceImplementation::Demo(device) => device.subscribe_to_state_updates().await,
        }
    }

    pub async fn name(&self) -> openscq30_lib::Result<String> {
        match self {
            DeviceImplementation::Manual(device) => device.name().await,
            DeviceImplementation::Demo(device) => device.name().await,
        }
    }

    pub async fn mac_address(&self) -> openscq30_lib::Result<MacAddr6> {
        match self {
            DeviceImplementation::Manual(device) => device.mac_address().await,
            DeviceImplementation::Demo(device) => device.mac_address().await,
        }
    }

    pub fn service_uuid(&self) -> Uuid {
        match self {
            DeviceImplementation::Manual(device) => device.service_uuid(),
            DeviceImplementation::Demo(device) => device.service_uuid(),
        }
    }

    pub async fn state(&self) -> DeviceState {
        match self {
            DeviceImplementation::Manual(device) => device.state().await,
            DeviceImplementation::Demo(device) => device.state().await,
        }
    }

    pub async fn set_sound_modes(&self, sound_modes: SoundModes) -> openscq30_lib::Result<()> {
        match self {
            DeviceImplementation::Manual(device) => device.set_sound_modes(sound_modes).await,
            DeviceImplementation::Demo(device) => device.set_sound_modes(sound_modes).await,
        }
    }

    pub async fn set_ambient_sound_mode_cycle(
        &self,
        cycle: AmbientSoundModeCycle,
    ) -> openscq30_lib::Result<()> {
        match self {
            DeviceImplementation::Manual(device) => {
                device.set_ambient_sound_mode_cycle(cycle).await
            }
            DeviceImplementation::Demo(device) => device.set_ambient_sound_mode_cycle(cycle).await,
        }
    }

    pub async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> openscq30_lib::Result<()> {
        match self {
            DeviceImplementation::Manual(device) => {
                device.set_equalizer_configuration(configuration).await
            }
            DeviceImplementation::Demo(device) => {
                device.set_equalizer_configuration(configuration).await
            }
        }
    }

    pub async fn set_hear_id(&self, hear_id: HearId) -> openscq30_lib::Result<()> {
        match self {
            DeviceImplementation::Manual(device) => device.set_hear_id(hear_id).await,
            DeviceImplementation::Demo(device) => device.set_hear_id(hear_id).await,
        }
    }

    pub async fn set_custom_button_model(
        &self,
        custom_button_model: CustomButtonModel,
    ) -> openscq30_lib::Result<()> {
        match self {
            DeviceImplementation::Manual(device) => {
                device.set_custom_button_model(custom_button_model).await
            }
            DeviceImplementation::Demo(device) => {
                device.set_custom_button_model(custom_button_model).await
            }
        }
    }
}

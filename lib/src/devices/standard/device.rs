use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;

use crate::{
    api::{
        connection::{RfcommBackend, RfcommConnection},
        device::{GenericDeviceDescriptor, OpenSCQ30Device, OpenSCQ30DeviceRegistry},
    },
    device_utils,
    soundcore_device::device_model::DeviceModel,
    storage::OpenSCQ30Database,
};

pub struct SoundcoreDeviceRegistry<B: RfcommBackend, D> {
    backend: B,
    database: Arc<OpenSCQ30Database>,
    device_model: DeviceModel,
    _device: PhantomData<D>,
}

impl<B: RfcommBackend, D> SoundcoreDeviceRegistry<B, D> {
    pub fn new(backend: B, database: Arc<OpenSCQ30Database>, device_model: DeviceModel) -> Self {
        Self {
            backend,
            device_model,
            database,
            _device: PhantomData,
        }
    }
}

#[async_trait]
impl<B, D> OpenSCQ30DeviceRegistry for SoundcoreDeviceRegistry<B, D>
where
    B: RfcommBackend + 'static + Send + Sync,
    D: SoundcoreDevice<B::ConnectionType> + OpenSCQ30Device + Send + Sync + 'static,
{
    async fn devices(&self) -> crate::Result<Vec<GenericDeviceDescriptor>> {
        self.backend.devices().await.map(|descriptors| {
            descriptors
                .into_iter()
                .map(|d| GenericDeviceDescriptor::new(d.name, d.mac_address))
                .collect()
        })
    }

    async fn connect(
        &self,
        mac_address: macaddr::MacAddr6,
    ) -> crate::Result<Arc<dyn OpenSCQ30Device + Send + Sync>> {
        let connection = self
            .backend
            .connect(mac_address, |addr| {
                addr.into_iter()
                    .find(device_utils::is_soundcore_vendor_rfcomm_uuid)
                    .unwrap_or(device_utils::RFCOMM_UUID)
            })
            .await?;
        let device = D::new(self.database.clone(), connection, self.device_model).await?;
        Ok(Arc::new(device))
    }
}

pub trait SoundcoreDevice<ConnectionType>
where
    Self: Sized,
    ConnectionType: RfcommConnection + 'static + Send + Sync,
{
    fn new(
        database: Arc<OpenSCQ30Database>,
        connection: ConnectionType,
        device_model: DeviceModel,
    ) -> impl Future<Output = crate::Result<Self>> + Send;
}

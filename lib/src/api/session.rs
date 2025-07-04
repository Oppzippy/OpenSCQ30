use std::{path::PathBuf, sync::Arc};

use macaddr::MacAddr6;

use crate::{
    connection_backend::{self, ConnectionBackends},
    devices::DeviceModel,
    storage::{OpenSCQ30Database, PairedDevice},
};

use super::{
    connection::ConnectionDescriptor,
    device::{self, OpenSCQ30Device},
    quick_presets::QuickPresetsHandler,
};

pub struct OpenSCQ30Session {
    database: Arc<OpenSCQ30Database>,
}

impl OpenSCQ30Session {
    pub async fn new(db_path: PathBuf) -> device::Result<Self> {
        Ok(Self {
            database: Arc::new(OpenSCQ30Database::new_file(db_path).await?),
        })
    }

    #[cfg(debug_assertions)]
    pub async fn new_with_in_memory_db() -> device::Result<Self> {
        Ok(Self {
            database: Arc::new(OpenSCQ30Database::new_in_memory().await?),
        })
    }

    pub async fn pair(&self, paired_device: PairedDevice) -> device::Result<()> {
        self.database
            .upsert_paired_device(paired_device)
            .await
            .map_err(Into::into)
    }

    pub async fn unpair(&self, mac_address: MacAddr6) -> device::Result<()> {
        self.database
            .delete_paired_device(mac_address)
            .await
            .map_err(Into::into)
    }

    pub async fn paired_devices(&self) -> device::Result<Vec<PairedDevice>> {
        self.database
            .fetch_all_paired_devices()
            .await
            .map_err(Into::into)
    }

    pub async fn list_devices(
        &self,
        model: DeviceModel,
    ) -> device::Result<Vec<ConnectionDescriptor>> {
        self.list_devices_with_backends(
            &connection_backend::default_backends().expect("no default backends available"),
            model,
        )
        .await
    }

    pub async fn list_devices_with_backends(
        &self,
        backends: &(impl ConnectionBackends + 'static),
        model: DeviceModel,
    ) -> device::Result<Vec<ConnectionDescriptor>> {
        model
            .device_registry(backends, self.database.clone())
            .await?
            .devices()
            .await
    }

    pub async fn list_demo_devices(
        &self,
        model: DeviceModel,
    ) -> device::Result<Vec<ConnectionDescriptor>> {
        model
            .demo_device_registry(self.database.clone())
            .await?
            .devices()
            .await
    }

    pub async fn connect(
        &self,
        mac_address: MacAddr6,
    ) -> device::Result<Arc<dyn OpenSCQ30Device + Send + Sync>> {
        self.connect_with_backends(
            &connection_backend::default_backends().expect("no default backends available"),
            mac_address,
        )
        .await
    }

    pub async fn connect_with_backends(
        &self,
        backends: &(impl ConnectionBackends + 'static),
        mac_address: MacAddr6,
    ) -> device::Result<Arc<dyn OpenSCQ30Device + Send + Sync>> {
        if let Some(paired_device) = self.database.fetch_paired_device(mac_address).await? {
            let registry = if paired_device.is_demo {
                paired_device
                    .model
                    .demo_device_registry(self.database.clone())
                    .await?
            } else {
                paired_device
                    .model
                    .device_registry(backends, self.database.clone())
                    .await?
            };
            registry.connect(mac_address).await
        } else {
            Err(device::Error::DeviceNotFound { mac_address })
        }
    }

    pub fn quick_preset_handler(&self) -> QuickPresetsHandler {
        QuickPresetsHandler::new(self.database.clone())
    }
}

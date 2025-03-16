use std::{path::PathBuf, sync::Arc};

use macaddr::MacAddr6;

use crate::{
    connection_backend::{self, ConnectionBackends},
    soundcore_device::device_model::DeviceModel,
    storage::{OpenSCQ30Database, PairedDevice},
};

use super::{
    device::{GenericDeviceDescriptor, OpenSCQ30Device},
    quick_presets::QuickPresetsHandler,
};

pub struct OpenSCQ30Session {
    database: Arc<OpenSCQ30Database>,
}

impl OpenSCQ30Session {
    pub async fn new(db_path: PathBuf) -> crate::Result<Self> {
        Ok(Self {
            database: Arc::new(OpenSCQ30Database::new_file(db_path).await?),
        })
    }

    pub async fn pair(&self, paired_device: PairedDevice) -> crate::Result<()> {
        self.database
            .upsert_paired_device(paired_device)
            .await
            .map_err(Into::into)
    }

    pub async fn unpair(&self, mac_address: MacAddr6) -> crate::Result<()> {
        self.database
            .delete_paired_device(mac_address)
            .await
            .map_err(Into::into)
    }

    pub async fn paired_devices(&self) -> crate::Result<Vec<PairedDevice>> {
        self.database
            .fetch_all_paired_devices()
            .await
            .map_err(Into::into)
    }

    pub async fn list_devices(
        &self,
        model: DeviceModel,
    ) -> crate::Result<Vec<GenericDeviceDescriptor>> {
        self.list_devices_with_backends(
            connection_backend::default_backends().expect("no default backends available"),
            model,
        )
        .await
    }

    pub async fn list_devices_with_backends(
        &self,
        backends: impl ConnectionBackends + 'static,
        model: DeviceModel,
    ) -> crate::Result<Vec<GenericDeviceDescriptor>> {
        model
            .device_registry(backends, self.database.clone(), true)
            .await?
            .devices()
            .await
    }

    pub async fn connect(
        &self,
        mac_address: MacAddr6,
    ) -> crate::Result<Arc<dyn OpenSCQ30Device + Send + Sync>> {
        self.connect_with_backends(
            connection_backend::default_backends().expect("no default backends available"),
            mac_address,
        )
        .await
    }

    pub async fn connect_with_backends(
        &self,
        backends: impl ConnectionBackends + 'static,
        mac_address: MacAddr6,
    ) -> crate::Result<Arc<dyn OpenSCQ30Device + Send + Sync>> {
        if let Some(paired_device) = self.database.fetch_paired_device(mac_address).await? {
            let registry = paired_device
                .model
                .device_registry(backends, self.database.clone(), true)
                .await?;
            registry.connect(mac_address).await
        } else {
            Err(crate::Error::DeviceNotFound { source: None })
        }
    }

    pub fn quick_preset_handler(&self) -> QuickPresetsHandler {
        QuickPresetsHandler::new(self.database.clone())
    }
}

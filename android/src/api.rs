use std::{path::PathBuf, sync::Arc};

use openscq30_lib::api::OpenSCQ30Session as LibSession;

use crate::{
    connection::ManualConnectionBackends, device::OpenSCQ30Device,
    quick_presets::QuickPresetsHandler, serializable,
};

#[derive(uniffi::Object)]
pub struct OpenSCQ30Session {
    inner: LibSession,
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn new_session(db_path: String) -> Result<OpenSCQ30Session, crate::Error> {
    let inner_session = LibSession::new(PathBuf::from(db_path)).await?;
    Ok(OpenSCQ30Session {
        inner: inner_session,
    })
}

#[uniffi::export(async_runtime = "tokio")]
impl OpenSCQ30Session {
    pub async fn pair(
        &self,
        paired_device: serializable::PairedDevice,
    ) -> Result<(), crate::Error> {
        self.inner.pair(paired_device.0).await?;
        Ok(())
    }

    pub async fn unpair(&self, mac_address: serializable::MacAddr6) -> Result<(), crate::Error> {
        self.inner.unpair(mac_address.0).await?;
        Ok(())
    }

    pub async fn paired_devices(&self) -> Result<Vec<serializable::PairedDevice>, crate::Error> {
        let devices = self.inner.paired_devices().await?;
        Ok(devices
            .into_iter()
            .map(serializable::PairedDevice)
            .collect())
    }

    pub async fn list_devices(
        &self,
        model: serializable::DeviceModel,
    ) -> Result<Vec<serializable::ConnectionDescriptor>, crate::Error> {
        let descriptors = self.inner.list_devices(model.0).await?;
        Ok(descriptors
            .into_iter()
            .map(serializable::ConnectionDescriptor)
            .collect())
    }

    pub async fn list_devices_with_backends(
        &self,
        backends: Arc<ManualConnectionBackends>,
        model_json: &str,
    ) -> Result<Vec<serializable::ConnectionDescriptor>, crate::Error> {
        let model = serde_json::from_str(model_json)?;
        let descriptors = self
            .inner
            .list_devices_with_backends(backends.as_ref(), model)
            .await?;
        Ok(descriptors
            .into_iter()
            .map(serializable::ConnectionDescriptor)
            .collect())
    }

    pub async fn connect(
        &self,
        mac_address: serializable::MacAddr6,
    ) -> Result<Arc<OpenSCQ30Device>, crate::Error> {
        self.inner
            .connect(mac_address.0)
            .await
            .map(OpenSCQ30Device::from)
            .map(Arc::new)
            .map_err(Into::into)
    }

    pub async fn connect_with_backends(
        &self,
        backends: Arc<ManualConnectionBackends>,
        mac_address: serializable::MacAddr6,
    ) -> Result<Arc<OpenSCQ30Device>, crate::Error> {
        self.inner
            .connect_with_backends(backends.as_ref(), mac_address.0)
            .await
            .map(OpenSCQ30Device::from)
            .map(Arc::new)
            .map_err(Into::into)
    }

    pub fn quick_preset_handler(&self) -> QuickPresetsHandler {
        QuickPresetsHandler {
            inner: self.inner.quick_preset_handler(),
        }
    }
}

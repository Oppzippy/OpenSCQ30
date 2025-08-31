use std::{path::PathBuf, sync::Arc};

use cfg_if::cfg_if;
use openscq30_lib::OpenSCQ30Session as LibSession;

use crate::{
    connection::ManualConnectionBackends, device::OpenSCQ30Device,
    quick_presets::QuickPresetsHandler, serializable,
};

#[derive(uniffi::Object)]
pub struct OpenSCQ30Session {
    inner: LibSession,
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn new_session(db_path: String) -> Result<OpenSCQ30Session, crate::OpenSCQ30Error> {
    let inner_session = LibSession::new(PathBuf::from(db_path)).await?;
    Ok(OpenSCQ30Session {
        inner: inner_session,
    })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn new_session_with_in_memory_db() -> Result<OpenSCQ30Session, crate::OpenSCQ30Error> {
    cfg_if! {
        if #[cfg(debug_assertions)] {
            let inner_session = LibSession::new_with_in_memory_db().await?;
            Ok(OpenSCQ30Session {
                inner: inner_session,
            })
        } else {
            unimplemented!()
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl OpenSCQ30Session {
    pub async fn pair(
        &self,
        paired_device: serializable::PairedDevice,
    ) -> Result<(), crate::OpenSCQ30Error> {
        self.inner.pair(paired_device.0).await?;
        Ok(())
    }

    pub async fn unpair(
        &self,
        mac_address: serializable::MacAddr6,
    ) -> Result<(), crate::OpenSCQ30Error> {
        self.inner.unpair(mac_address.0).await?;
        Ok(())
    }

    pub async fn paired_devices(
        &self,
    ) -> Result<Vec<serializable::PairedDevice>, crate::OpenSCQ30Error> {
        let devices = self.inner.paired_devices().await?;
        Ok(devices
            .into_iter()
            .map(serializable::PairedDevice)
            .collect())
    }

    pub async fn list_demo_devices(
        &self,
        model: serializable::DeviceModel,
    ) -> Result<Vec<serializable::ConnectionDescriptor>, crate::OpenSCQ30Error> {
        let descriptors = self.inner.list_demo_devices(model.0).await?;
        Ok(descriptors
            .into_iter()
            .map(serializable::ConnectionDescriptor)
            .collect())
    }

    pub async fn list_devices_with_backends(
        &self,
        backends: Arc<ManualConnectionBackends>,
        model: serializable::DeviceModel,
    ) -> Result<Vec<serializable::ConnectionDescriptor>, crate::OpenSCQ30Error> {
        let descriptors = self
            .inner
            .list_devices_with_backends(backends.as_ref(), model.0)
            .await?;
        Ok(descriptors
            .into_iter()
            .map(serializable::ConnectionDescriptor)
            .collect())
    }

    pub async fn connect_with_backends(
        &self,
        backends: Arc<ManualConnectionBackends>,
        mac_address: serializable::MacAddr6,
    ) -> Result<Arc<OpenSCQ30Device>, crate::OpenSCQ30Error> {
        let device = self
            .inner
            .connect_with_backends(backends.as_ref(), mac_address.0)
            .await?;
        let wrapped_device = OpenSCQ30Device::new(device).await;
        Ok(Arc::new(wrapped_device))
    }

    pub fn quick_preset_handler(&self) -> QuickPresetsHandler {
        QuickPresetsHandler {
            inner: self.inner.quick_preset_handler(),
        }
    }
}

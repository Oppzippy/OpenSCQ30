use std::collections::HashMap;

use crate::{device::OpenSCQ30Device, serializable};

#[derive(uniffi::Object)]
pub struct QuickPresetsHandler {
    pub inner: openscq30_lib::api::quick_presets::QuickPresetsHandler,
}

#[uniffi::export(async_runtime = "tokio")]
impl QuickPresetsHandler {
    #[uniffi::constructor]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        unimplemented!()
    }

    pub async fn quick_presets(
        &self,
        device: &OpenSCQ30Device,
    ) -> Result<Vec<serializable::QuickPreset>, crate::Error> {
        let quick_presets = self.inner.quick_presets(device.inner.as_ref()).await?;
        Ok(quick_presets
            .into_iter()
            .map(serializable::QuickPreset)
            .collect())
    }

    pub async fn save(
        &self,
        device: &OpenSCQ30Device,
        name: String,
        settings: HashMap<serializable::SettingId, serializable::Value>,
    ) -> Result<(), crate::Error> {
        self.inner
            .save(
                device.inner.as_ref(),
                name,
                settings
                    .into_iter()
                    .map(|(id, value)| (id.0, value.0))
                    .collect(),
            )
            .await
            .map_err(Into::into)
    }

    pub async fn activate(&self, device: &OpenSCQ30Device, name: &str) -> Result<(), crate::Error> {
        self.inner
            .activate(device.inner.as_ref(), name)
            .await
            .map_err(Into::into)
    }

    pub async fn delete(&self, device: &OpenSCQ30Device, name: String) -> Result<(), crate::Error> {
        self.inner
            .activate(device.inner.as_ref(), name)
            .await
            .map_err(Into::into)
    }
}

use crate::{device::OpenSCQ30Device, serializable};

#[derive(uniffi::Object)]
pub struct QuickPresetsHandler {
    pub inner: openscq30_lib::quick_presets::QuickPresetsHandler,
}

#[uniffi::export(async_runtime = "tokio")]
impl QuickPresetsHandler {
    pub async fn quick_presets(
        &self,
        device: &OpenSCQ30Device,
    ) -> Result<Vec<serializable::QuickPreset>, crate::OpenSCQ30Error> {
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
    ) -> Result<(), crate::OpenSCQ30Error> {
        self.inner
            .save(device.inner.as_ref(), name)
            .await
            .map_err(Into::into)
    }

    pub async fn toggle_field(
        &self,
        device: &OpenSCQ30Device,
        name: String,
        setting_id: serializable::SettingId,
        is_enabled: bool,
    ) -> Result<(), crate::OpenSCQ30Error> {
        self.inner
            .toggle_field(device.inner.as_ref(), name, setting_id.0, is_enabled)
            .await
            .map_err(Into::into)
    }

    pub async fn activate(
        &self,
        device: &OpenSCQ30Device,
        name: &str,
    ) -> Result<(), crate::OpenSCQ30Error> {
        self.inner
            .activate(device.inner.as_ref(), name)
            .await
            .map_err(Into::into)
    }

    pub async fn delete(
        &self,
        device: &OpenSCQ30Device,
        name: String,
    ) -> Result<(), crate::OpenSCQ30Error> {
        self.inner
            .activate(device.inner.as_ref(), name)
            .await
            .map_err(Into::into)
    }
}

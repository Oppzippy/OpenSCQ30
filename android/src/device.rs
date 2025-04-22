use std::sync::Arc;

use openscq30_lib::api::device::OpenSCQ30Device as LibOpenSCQ30Device;

use crate::serializable;

#[derive(uniffi::Object)]
pub struct OpenSCQ30Device {
    pub inner: Arc<dyn LibOpenSCQ30Device + Send + Sync>,
}

impl From<Arc<dyn LibOpenSCQ30Device + Send + Sync>> for OpenSCQ30Device {
    fn from(inner: Arc<dyn LibOpenSCQ30Device + Send + Sync>) -> Self {
        Self { inner }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl OpenSCQ30Device {
    fn model(&self) -> serializable::DeviceModel {
        serializable::DeviceModel(self.inner.model())
    }

    fn categories(&self) -> Vec<serializable::CategoryId> {
        self.inner
            .categories()
            .into_iter()
            .map(serializable::CategoryId)
            .collect()
    }

    fn settings_in_category(
        &self,
        category_id: serializable::CategoryId,
    ) -> Vec<serializable::SettingId> {
        self.inner
            .settings_in_category(&category_id.0)
            .into_iter()
            .map(serializable::SettingId)
            .collect()
    }

    fn setting(&self, setting_id: serializable::SettingId) -> Option<serializable::Setting> {
        self.inner.setting(&setting_id.0).map(serializable::Setting)
    }

    async fn set_setting_values(
        &self,
        setting_values: Vec<SettingIdValuePair>,
    ) -> Result<(), crate::Error> {
        self.inner
            .set_setting_values(
                setting_values
                    .into_iter()
                    .map(|pair| (pair.setting.0, pair.value.0))
                    .collect(),
            )
            .await
            .map_err(Into::into)
    }
}

#[derive(uniffi::Record)]
pub struct SettingIdValuePair {
    setting: serializable::SettingId,
    value: serializable::Value,
}

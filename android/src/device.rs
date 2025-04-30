use std::sync::{Arc, Mutex};

use openscq30_lib::api::device::OpenSCQ30Device as LibOpenSCQ30Device;

use crate::serializable;

#[derive(uniffi::Object)]
pub struct OpenSCQ30Device {
    pub inner: Arc<dyn LibOpenSCQ30Device + Send + Sync>,
    connection_status_callback: Arc<Mutex<Option<Arc<dyn ConnectionStatusCallback>>>>,
    watch_for_changes_callback: Arc<Mutex<Option<Arc<dyn NotificationCallback>>>>,
}

impl OpenSCQ30Device {
    pub async fn new(inner: Arc<dyn LibOpenSCQ30Device + Send + Sync>) -> Self {
        let connection_status_callback: Arc<Mutex<Option<Arc<dyn ConnectionStatusCallback>>>> =
            Default::default();
        {
            let connection_status_callback = connection_status_callback.clone();
            let inner = inner.clone();
            tokio::spawn(async move {
                loop {
                    if inner.connection_status().changed().await.is_err() {
                        break;
                    }
                    if let Some(callback) = connection_status_callback.lock().unwrap().as_ref() {
                        callback.on_change(serializable::ConnectionStatus(
                            inner.connection_status().borrow().to_owned(),
                        ));
                    }
                }
            });
        }
        let watch_for_changes_callback: Arc<Mutex<Option<Arc<dyn NotificationCallback>>>> =
            Default::default();
        {
            let watch_for_changes_callback = watch_for_changes_callback.clone();
            let inner = inner.clone();
            tokio::spawn(async move {
                loop {
                    if inner.watch_for_changes().changed().await.is_err() {
                        break;
                    }
                    if let Some(callback) = watch_for_changes_callback.lock().unwrap().as_ref() {
                        callback.on_notify();
                    }
                }
            });
        }
        Self {
            inner,
            connection_status_callback,
            watch_for_changes_callback,
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl OpenSCQ30Device {
    pub fn set_connection_status_callback(&self, callback: Arc<dyn ConnectionStatusCallback>) {
        *self.connection_status_callback.lock().unwrap() = Some(callback);
    }

    fn set_watch_for_changes_callback(&self, callback: Arc<dyn NotificationCallback>) {
        *self.watch_for_changes_callback.lock().unwrap() = Some(callback)
    }

    pub fn model(&self) -> serializable::DeviceModel {
        serializable::DeviceModel(self.inner.model())
    }

    pub fn categories(&self) -> Vec<serializable::CategoryId> {
        self.inner
            .categories()
            .into_iter()
            .map(serializable::CategoryId)
            .collect()
    }

    pub fn settings_in_category(
        &self,
        category_id: serializable::CategoryId,
    ) -> Vec<serializable::SettingId> {
        self.inner
            .settings_in_category(&category_id.0)
            .into_iter()
            .map(serializable::SettingId)
            .collect()
    }

    pub fn setting(&self, setting_id: serializable::SettingId) -> Option<serializable::Setting> {
        self.inner.setting(&setting_id.0).map(serializable::Setting)
    }

    pub async fn set_setting_values(
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

#[uniffi::export(with_foreign)]
pub trait ConnectionStatusCallback: Send + Sync {
    fn on_change(&self, connection_status: serializable::ConnectionStatus);
}

#[uniffi::export(with_foreign)]
pub trait NotificationCallback: Send + Sync {
    fn on_notify(&self);
}

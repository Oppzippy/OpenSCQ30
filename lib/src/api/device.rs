use std::{panic::Location, sync::Arc};

use async_trait::async_trait;
use indexmap::IndexMap;
use macaddr::MacAddr6;
use tokio::sync::watch;

use crate::{devices::DeviceModel, macros::impl_from_source_error_with_location, storage};

use super::{
    connection::{self, ConnectionDescriptor, ConnectionStatus},
    settings::{self, CategoryId, Setting, SettingId, Value},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("connection")]
    ConnectionError {
        source: connection::Error,
        location: &'static Location<'static>,
    },
    #[error("storage")]
    StorageError {
        source: storage::Error,
        location: &'static Location<'static>,
    },
    #[error("setting")]
    SettingError {
        #[from]
        source: settings::Error,
    },
    #[error("other")]
    Other {
        source: Box<dyn std::error::Error + Send + Sync>,
        location: &'static Location<'static>,
    },

    #[error("{action} timed out")]
    ActionTimedOut { action: &'static str },
    #[error("device with mac address {mac_address} not found")]
    DeviceNotFound { mac_address: MacAddr6 },
}
pub type Result<T> = std::result::Result<T, Error>;

impl_from_source_error_with_location!(Error::ConnectionError(connection::Error));
impl_from_source_error_with_location!(Error::StorageError(storage::Error));
impl_from_source_error_with_location!(Error::Other(Box<dyn std::error::Error + Send + Sync>));

impl Error {
    #[track_caller]
    pub fn other<E: std::error::Error + Send + Sync + 'static>(error: E) -> Self {
        (Box::new(error) as Box<dyn std::error::Error + Send + Sync>).into()
    }
}

#[async_trait]
pub trait OpenSCQ30DeviceRegistry {
    /// Lists a superset of the connectable devices. For example, this may list all bluetooth devices rather than only
    /// Soundcore devices in the case of SoundcoreDeviceRegistry.
    async fn devices(&self) -> Result<Vec<ConnectionDescriptor>>;

    /// Connects to the device and requests any data necessary for initialization.
    async fn connect(
        &self,
        mac_address: MacAddr6,
    ) -> Result<Arc<dyn OpenSCQ30Device + Send + Sync>>;
}

#[async_trait]
pub trait OpenSCQ30Device {
    /// Returns a tokio::sync::watch::Receiver for tracking when the connection disconnects.
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;

    /// Returns the model of the device.
    fn model(&self) -> DeviceModel;

    /// Returns an ordered map of `CaregoryId`s to an ordered map of `SettingId`s to `Setting`s. The categories and
    /// settings should be displayed in order when presented to the user.
    ///
    /// Example (as json for demonstration purposes, pretend json objects are ordered):
    /// ```jsonc
    /// {
    ///     "general": {
    ///         "ambientSoundMode": { /* the setting */},
    ///         "noiseCancelingMode": { /* the setting */},
    ///     },
    ///     "equalizer": {
    ///         // ...
    ///     }
    /// }
    /// ```
    fn settings_by_category(&self) -> IndexMap<CategoryId, IndexMap<SettingId, Setting>> {
        self.categories()
            .into_iter()
            .map(|category_id| {
                (
                    category_id,
                    self.settings_in_category(&category_id)
                        .into_iter()
                        .filter_map(|setting_id| {
                            self.setting(&setting_id)
                                .map(|setting| (setting_id, setting))
                        })
                        .collect(),
                )
            })
            .collect()
    }

    /// Returns all relevant category ids for the device. Each included category id will have 0 or more child setting
    /// ids. The categories should be displayed in order when presented to the user.
    fn categories(&self) -> Vec<CategoryId>;

    /// Returns all relevant `SettingId`s in the specified category. Each `SettingId` may or may not actually be present.
    fn settings_in_category(&self, category_id: &CategoryId) -> Vec<SettingId>;

    /// Returns the setting for a given `SettingId`, or None if:
    /// 1. The `SettingId` is relevant to the device, but there is not currently a setting present (but there may be in
    ///    in the future).
    /// 2. The `SettingId` is not relevant to the device, so a setting will never be present.
    fn setting(&self, setting_id: &SettingId) -> Option<Setting>;

    /// This returns a `tokio::sync::watch::Receiver` that will fire changed whenever any setting changes.
    fn watch_for_changes(&self) -> watch::Receiver<()>;

    /// Sets many setting values in one go. This can enable optimizations unavailable when setting them individually.
    ///
    /// For example, if the device has a command that makes multiple changes at once, that command can be sent only a
    /// single time rather than once for each change.
    async fn set_setting_values(&self, setting_values: Vec<(SettingId, Value)>) -> Result<()>;
}

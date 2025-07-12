use std::sync::Arc;

use tokio::sync::watch;
use tracing::warn;

use crate::{
    devices::DeviceModel,
    storage::{self, OpenSCQ30Database},
};

pub struct CustomEqualizerProfileStore {
    database: Arc<OpenSCQ30Database>,
    sender: watch::Sender<Vec<(String, Vec<i16>)>>,
    device_model: DeviceModel,
}

impl CustomEqualizerProfileStore {
    pub async fn new(
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        change_notify: watch::Sender<()>,
    ) -> Self {
        let initial_profiles = database
            .fetch_all_equalizer_profiles(device_model)
            .await
            .unwrap_or_else(|err| {
                warn!("error fetching custom equalizer profiles, continuing without them: {err:?}");
                Vec::new()
            });
        let (sender, mut receiver) = watch::channel(initial_profiles);
        // As long as we don't allow anyone outside this struct to acquire a copy of sender, sender will be dropped
        // at the same time as this struct, causing receiver.changed() to error and async task to end.
        tokio::spawn(async move {
            while receiver.changed().await.is_ok() {
                change_notify.send_replace(());
            }
        });
        Self {
            database,
            sender,
            device_model,
        }
    }

    pub fn subscribe(&self) -> watch::Receiver<Vec<(String, Vec<i16>)>> {
        self.sender.subscribe()
    }

    pub async fn insert(&self, name: String, volume_adjustments: Vec<i16>) -> storage::Result<()> {
        self.database
            .upsert_equalizer_profile(self.device_model, name, volume_adjustments)
            .await?;
        self.refresh().await?;
        Ok(())
    }

    pub async fn delete(&self, name: String) -> storage::Result<()> {
        self.database
            .delete_equalizer_profile(self.device_model, name)
            .await?;
        self.refresh().await?;
        Ok(())
    }

    async fn refresh(&self) -> storage::Result<()> {
        let profiles = self
            .database
            .fetch_all_equalizer_profiles(self.device_model)
            .await?;
        self.sender.send_replace(profiles);
        Ok(())
    }
}

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::soundcore_bluetooth::traits::{
    SoundcoreDeviceConnectionError, SoundcoreDeviceConnectionRegistry,
};

use super::StubSoundcoreDeviceConnection;

#[derive(Debug)]
pub struct StubSoundcoreDeviceConnectionRegistry {
    connections: Vec<Arc<<Self as SoundcoreDeviceConnectionRegistry>::DeviceConnectionType>>,
    refresh_connections_return: RwLock<Option<Result<(), SoundcoreDeviceConnectionError>>>,
}

impl StubSoundcoreDeviceConnectionRegistry {
    pub fn new(
        connections: Vec<Arc<<Self as SoundcoreDeviceConnectionRegistry>::DeviceConnectionType>>,
    ) -> Self {
        Self {
            connections,
            refresh_connections_return: RwLock::new(Some(Ok(()))),
        }
    }

    pub async fn set_refresh_connections_return(
        &self,
        refresh_connections_return: Result<(), SoundcoreDeviceConnectionError>,
    ) {
        let mut lock = self.refresh_connections_return.write().await;
        *lock = Some(refresh_connections_return);
    }
}

#[async_trait]
impl SoundcoreDeviceConnectionRegistry for StubSoundcoreDeviceConnectionRegistry {
    type DeviceConnectionType = StubSoundcoreDeviceConnection;

    async fn refresh_connections(&self) -> Result<(), SoundcoreDeviceConnectionError> {
        self.refresh_connections_return
            .write()
            .await
            .take()
            .unwrap()
    }

    async fn connections(&self) -> Vec<Arc<Self::DeviceConnectionType>> {
        self.connections.to_owned()
    }
}

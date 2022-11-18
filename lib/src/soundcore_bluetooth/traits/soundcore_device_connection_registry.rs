use std::{error::Error, sync::Arc};

use async_trait::async_trait;

use super::soundcore_device_connection::SoundcoreDeviceConnection;

#[async_trait]
pub trait SoundcoreDeviceConnectionRegistry {
    async fn refresh_connections(&mut self) -> Result<(), Box<dyn Error>>;
    async fn get_connections(&self) -> Vec<Arc<dyn SoundcoreDeviceConnection + Sync + Send>>;
}

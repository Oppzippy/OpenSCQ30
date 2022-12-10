use async_trait::async_trait;
use tokio::sync::mpsc;

use super::soundcore_device_connection_error::SoundcoreDeviceConnectionError;

#[async_trait]
pub trait SoundcoreDeviceConnection {
    async fn name(&self) -> Result<String, SoundcoreDeviceConnectionError>;
    async fn mac_address(&self) -> Result<String, SoundcoreDeviceConnectionError>;
    async fn write_with_response(&self, data: &[u8]) -> Result<(), SoundcoreDeviceConnectionError>;
    async fn write_without_response(
        &self,
        data: &[u8],
    ) -> Result<(), SoundcoreDeviceConnectionError>;
    async fn inbound_packets_channel(
        &self,
    ) -> Result<mpsc::Receiver<Vec<u8>>, SoundcoreDeviceConnectionError>;
}

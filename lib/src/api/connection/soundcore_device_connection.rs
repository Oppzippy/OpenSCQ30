use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait SoundcoreDeviceConnection {
    async fn name(&self) -> crate::Result<String>;
    async fn mac_address(&self) -> crate::Result<String>;
    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()>;
    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()>;
    async fn inbound_packets_channel(&self) -> crate::Result<mpsc::Receiver<Vec<u8>>>;
}

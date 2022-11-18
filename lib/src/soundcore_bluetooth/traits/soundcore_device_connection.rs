use std::error::Error;

use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::packets::inbound::inbound_packet::InboundPacket;

use super::soundcore_device_connection_error::SoundcoreDeviceError;

#[async_trait]
pub trait SoundcoreDeviceConnection {
    async fn get_name(&self) -> Result<String, Box<dyn Error>>;
    async fn get_mac_address(&self) -> Result<String, Box<dyn Error>>;
    async fn write_with_response(&self, data: &[u8]) -> Result<(), Box<dyn Error>>;
    async fn write_without_response(&self, data: &[u8]) -> Result<(), Box<dyn Error>>;
    async fn inbound_packets_channel(
        &self,
    ) -> Result<mpsc::Receiver<InboundPacket>, SoundcoreDeviceError>;
}

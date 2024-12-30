use macaddr::MacAddr6;
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

use super::ConnectionStatus;

pub trait Connection {
    async fn name(&self) -> crate::Result<String>;
    async fn mac_address(&self) -> crate::Result<MacAddr6>;
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;
    async fn write_with_response(&self, data: &[u8]) -> crate::Result<()>;
    async fn write_without_response(&self, data: &[u8]) -> crate::Result<()>;
    async fn inbound_packets_channel(&self) -> crate::Result<mpsc::Receiver<Vec<u8>>>;
    // TODO remove in v2
    fn service_uuid(&self) -> Uuid;
}

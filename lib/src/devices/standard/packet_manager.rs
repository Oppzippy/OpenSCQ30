use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::watch;
use tracing::warn;

use crate::soundcore_device::device::Packet;

use super::structures::Command;

pub struct PacketManager<T> {
    handlers: HashMap<Command, Box<dyn PacketHandler<T> + Send + Sync>>,
}

impl<T> Default for PacketManager<T> {
    fn default() -> Self {
        Self {
            handlers: Default::default(),
        }
    }
}

impl<T> PacketManager<T> {
    pub fn set_handler(
        &mut self,
        command: Command,
        handler: Box<dyn PacketHandler<T> + Send + Sync>,
    ) {
        self.handlers.insert(command, handler);
    }

    pub async fn handle(
        &self,
        state_sender: &watch::Sender<T>,
        packet: &Packet,
    ) -> crate::Result<()> {
        if let Some(handler) = self.handlers.get(&packet.command) {
            handler.handle_packet(state_sender, packet).await?;
        } else {
            warn!("no handler found for inbound packet: {packet:?}");
        }
        Ok(())
    }
}

#[async_trait(?Send)]
pub trait PacketHandler<T> {
    async fn handle_packet(&self, state: &watch::Sender<T>, packet: &Packet) -> crate::Result<()>;
}

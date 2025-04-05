use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::connection::Connection,
    devices::soundcore::{
        a3936::{
            packets::A3936SetMultiButtonConfigurationPacket,
            structures::A3936InternalMultiButtonConfiguration,
        },
        standard::{
            packets::packet_io_controller::PacketIOController, state_modifier::StateModifier,
        },
    },
};

pub struct ButtonConfigurationStateModifier<ConnectionType: Connection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: Connection> ButtonConfigurationStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for ButtonConfigurationStateModifier<ConnectionType>
where
    T: AsMut<A3936InternalMultiButtonConfiguration>
        + AsRef<A3936InternalMultiButtonConfiguration>
        + Clone
        + Send
        + Sync,
    ConnectionType: Connection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> crate::Result<()> {
        let target_button_config = target_state.as_ref();
        {
            let state = state_sender.borrow();
            let button_config = state.as_ref();
            if button_config == target_button_config {
                return Ok(());
            }
        }

        self.packet_io
            .send(&A3936SetMultiButtonConfigurationPacket::new(*target_button_config).into())
            .await?;
        state_sender.send_modify(|state| *state.as_mut() = *target_button_config);
        Ok(())
    }
}

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3959::{
            packets::A3959SetMultiButtonConfigurationPacket,
            structures::A3959MultiButtonConfiguration,
        },
        standard::{
            packets::packet_io_controller::PacketIOController, state_modifier::StateModifier,
        },
    },
};

pub struct ButtonConfigurationStateModifier<ConnectionType: RfcommConnection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection> ButtonConfigurationStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionType, T> StateModifier<T> for ButtonConfigurationStateModifier<ConnectionType>
where
    T: AsMut<A3959MultiButtonConfiguration>
        + AsRef<A3959MultiButtonConfiguration>
        + Clone
        + Send
        + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_button_config = target_state.as_ref();
        {
            let state = state_sender.borrow();
            let button_config = state.as_ref();
            if button_config == target_button_config {
                return Ok(());
            }
        }

        self.packet_io
            .send(&A3959SetMultiButtonConfigurationPacket::new(*target_button_config).into())
            .await?;
        state_sender.send_modify(|state| *state.as_mut() = *target_button_config);
        Ok(())
    }
}

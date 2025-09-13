use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3948,
        common::{packet::PacketIOController, state_modifier::StateModifier},
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
    T: Has<a3948::structures::MultiButtonConfiguration> + Clone + Send + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_button_config = target_state.get();
        {
            let state = state_sender.borrow();
            let button_config = state.get();
            if button_config == target_button_config {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(
                &a3948::packets::outbound::MultiButtonConfiguration::new(*target_button_config)
                    .into(),
            )
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target_button_config);
        Ok(())
    }
}

use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3035,
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
    T: Has<a3035::structures::ButtonConfiguration> + Clone + Send + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let button_configuration = *state_sender.borrow().get();
        let target_button_configuration = target_state.get();
        if &button_configuration == target_button_configuration {
            return Ok(());
        }

        self.packet_io
            .send_with_response(&a3035::packets::outbound::set_button_double_press_action(
                target_button_configuration.double_press_action,
            ))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target_button_configuration);

        Ok(())
    }
}

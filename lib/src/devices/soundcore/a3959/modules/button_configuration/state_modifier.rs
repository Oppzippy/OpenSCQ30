use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3959,
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
    T: Has<a3959::structures::MultiButtonConfiguration> + Clone + Send + Sync,
    ConnectionType: RfcommConnection + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_button_config = target_state.get();
        {
            let button_config = *state_sender.borrow().get();
            let changed_buttons = target_button_config
                .iterate_buttons()
                .filter(|(button, action)| *action != button_config.get_button(*button));

            for (button, action) in changed_buttons {
                tracing::debug!("{button:?} changed to {action:?}, sending packet");
                self.packet_io
                    .send_with_response(
                        &a3959::packets::outbound::ButtonConfiguration::new(button, action).into(),
                    )
                    .await?;
                state_sender.send_modify(|state| *state.get_mut().get_button_mut(button) = action);
            }
        }

        Ok(())
    }
}

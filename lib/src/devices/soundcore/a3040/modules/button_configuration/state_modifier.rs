use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3040,
        common::{packet::PacketIOController, state_modifier::StateModifier},
    },
};

pub struct ButtonConfigurationStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl ButtonConfigurationStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for ButtonConfigurationStateModifier
where
    T: Has<a3040::structures::ButtonConfiguration> + Clone + Send + Sync,
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
            .send_with_response(&a3040::packets::set_button_double_press_action(
                target_button_configuration.double_press_action,
            ))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target_button_configuration);

        Ok(())
    }
}

use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3954,
        common::{packet::PacketIOController, state_modifier::StateModifier},
    },
};

pub struct EasyChatStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl EasyChatStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for EasyChatStateModifier
where
    T: Has<a3954::structures::EasyChat> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target = target_state.get();
        {
            let state = state_sender.borrow();
            let current = state.get();
            if current == target {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(&a3954::packets::outbound::set_easy_chat(target))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target);
        Ok(())
    }
}

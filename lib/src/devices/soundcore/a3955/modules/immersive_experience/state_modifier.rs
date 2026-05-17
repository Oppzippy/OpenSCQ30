use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3955::{self, structures::ImmersiveExperience},
        common::{packet::PacketIOController, state_modifier::StateModifier},
    },
};

pub struct ImmersiveExperienceStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl ImmersiveExperienceStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<StateT> StateModifier<StateT> for ImmersiveExperienceStateModifier
where
    StateT: Has<ImmersiveExperience> + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
    ) -> device::Result<()> {
        {
            let state = state_sender.borrow();
            if state.get() == target_state.get() {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(&a3955::packets::outbound::set_immersive_experience(
                *target_state.get(),
            ))
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = *target_state.get();
        });
        Ok(())
    }
}

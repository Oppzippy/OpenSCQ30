use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        common::{packet::PacketIOController, state_modifier::StateModifier},
        d1301::{self, structures::AutoStopTimer},
    },
};

pub struct AutoStopTimerStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl AutoStopTimerStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<StateT> StateModifier<StateT> for AutoStopTimerStateModifier
where
    StateT: Has<AutoStopTimer> + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
    ) -> device::Result<()> {
        if state_sender.borrow().get() == target_state.get() {
            return Ok(());
        }

        self.packet_io
            .send_with_response(&d1301::packets::outbound::set_auto_stop_timer(
                target_state.get(),
            ))
            .await?;
        state_sender.send_modify(|state| {
            *state.get_mut() = *target_state.get();
        });
        Ok(())
    }
}

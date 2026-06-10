use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        packet::{self, PacketIOController},
        state_modifier::StateModifier,
        structures::{AmbientSoundModeCycleTws, TwsStatus},
    },
};

pub struct AmbientSoundModeCycleTwsStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl AmbientSoundModeCycleTwsStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for AmbientSoundModeCycleTwsStateModifier
where
    T: Has<TwsStatus>
        + Has<AmbientSoundModeCycleTws>
        + Has<ResetButtonConfigurationPending>
        + Clone
        + Send
        + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        // If we are resetting buttons to default, don't immediately put them back as they were afterwards
        let is_reset_pending: ResetButtonConfigurationPending = *target_state.get();
        if is_reset_pending.0 {
            return Ok(());
        }

        let target_cycle = target_state.get();
        {
            let state = state_sender.borrow();
            let cycle: &AmbientSoundModeCycleTws = state.get();
            if cycle == target_cycle {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(&packet::outbound::set_ambient_sound_mode_cycle_tws(
                *target_cycle,
            ))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = *target_cycle);
        Ok(())
    }
}

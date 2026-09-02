use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        common::{packet::PacketIOController, state_modifier::StateModifier},
        d1301::{
            self,
            structures::{AutoStopTimer, AutoSwitchOnceAsleep},
        },
    },
};

pub struct AutoSwitchOnceAsleepStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl AutoSwitchOnceAsleepStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<StateT> StateModifier<StateT> for AutoSwitchOnceAsleepStateModifier
where
    StateT: Has<AutoSwitchOnceAsleep> + Has<AutoStopTimer> + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
    ) -> device::Result<()> {
        let (enabled, post_sleep_audio) = {
            let state = state_sender.borrow();
            let enabled: &AutoSwitchOnceAsleep = state.get();
            let timer: &AutoStopTimer = state.get();
            (*enabled, timer.post_sleep_audio)
        };
        let target_enabled: AutoSwitchOnceAsleep = *target_state.get();
        let target_timer: &AutoStopTimer = target_state.get();
        let target_post_sleep_audio = target_timer.post_sleep_audio;

        // The official app enables the feature before setting the action.
        if enabled != target_enabled {
            self.packet_io
                .send_with_response(&d1301::packets::outbound::set_auto_switch_once_asleep(
                    target_enabled,
                ))
                .await?;
            state_sender.send_modify(|state| {
                *state.get_mut() = target_enabled;
            });
        }

        if post_sleep_audio != target_post_sleep_audio {
            self.packet_io
                .send_with_response(&d1301::packets::outbound::set_post_sleep_audio(
                    target_post_sleep_audio,
                ))
                .await?;
            state_sender.send_modify(|state| {
                let timer: &mut AutoStopTimer = state.get_mut();
                timer.post_sleep_audio = target_post_sleep_audio;
            });
        }

        Ok(())
    }
}

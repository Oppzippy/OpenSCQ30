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
            structures::{DefaultListeningMode, ListeningMode},
        },
    },
};

pub struct ListeningModeStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl ListeningModeStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<StateT> StateModifier<StateT> for ListeningModeStateModifier
where
    StateT: Has<ListeningMode> + Has<DefaultListeningMode> + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
    ) -> device::Result<()> {
        let (listening_mode, default_listening_mode): (ListeningMode, DefaultListeningMode) = {
            let state = state_sender.borrow();
            (*state.get(), *state.get())
        };
        let target_listening_mode: ListeningMode = *target_state.get();
        let target_default_listening_mode: DefaultListeningMode = *target_state.get();

        if listening_mode != target_listening_mode {
            self.packet_io
                .send_with_response(&d1301::packets::outbound::set_listening_mode(
                    target_listening_mode,
                ))
                .await?;
            state_sender.send_modify(|state| {
                *state.get_mut() = target_listening_mode;
            });
        }

        if default_listening_mode != target_default_listening_mode {
            self.packet_io
                .send_with_response(&d1301::packets::outbound::set_default_listening_mode(
                    target_default_listening_mode,
                ))
                .await?;
            state_sender.send_modify(|state| {
                *state.get_mut() = target_default_listening_mode;
            });
        }

        Ok(())
    }
}

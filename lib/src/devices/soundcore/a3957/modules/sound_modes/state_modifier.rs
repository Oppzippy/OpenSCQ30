use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::{connection::RfcommConnection, device},
    devices::soundcore::{
        a3957::{
            self,
            structures::{AncPersonalizedToEarCanal, SoundModes},
        },
        common::{packet::PacketIOController, state_modifier::StateModifier},
    },
};

pub struct AncPersonalizedToEarCanalStateModifier<ConnectionType: RfcommConnection> {
    packet_io: Arc<PacketIOController<ConnectionType>>,
}

impl<ConnectionType: RfcommConnection> AncPersonalizedToEarCanalStateModifier<ConnectionType> {
    pub fn new(packet_io: Arc<PacketIOController<ConnectionType>>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<ConnectionT, StateT> StateModifier<StateT>
    for AncPersonalizedToEarCanalStateModifier<ConnectionT>
where
    ConnectionT: RfcommConnection + Send + Sync,
    StateT: Has<SoundModes> + Has<AncPersonalizedToEarCanal> + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<StateT>,
        target_state: &StateT,
    ) -> device::Result<()> {
        {
            let state = state_sender.borrow();
            let current: &AncPersonalizedToEarCanal = state.get();
            if current == target_state.get() {
                return Ok(());
            }
        }

        self.packet_io
            .send_with_response(
                &a3957::packets::outbound::set_anc_personalized_to_hear_canal(target_state.get()),
            )
            .await?;
        state_sender.send_modify(|state| {
            let current: &mut AncPersonalizedToEarCanal = state.get_mut();
            *current = *target_state.get();
        });
        Ok(())
    }
}

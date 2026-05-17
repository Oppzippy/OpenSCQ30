use async_trait::async_trait;
use openscq30_lib_has::Has;
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3116,
        common::{packet::PacketIOController, state_modifier::StateModifier},
    },
};

pub struct PowerOffStateModifier {
    packet_io: Arc<PacketIOController>,
}

impl PowerOffStateModifier {
    pub fn new(packet_io: Arc<PacketIOController>) -> Self {
        Self { packet_io }
    }
}

#[async_trait]
impl<T> StateModifier<T> for PowerOffStateModifier
where
    T: Has<a3116::structures::PowerOffPending> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        if *target_state.get() == a3116::structures::PowerOffPending(true) {
            self.packet_io
                .send_with_response(&a3116::packets::outbound::power_off())
                .await?;
            state_sender.send_modify(|state| {
                *state.get_mut() = a3116::structures::PowerOffPending::default();
            });
        }

        Ok(())
    }
}
